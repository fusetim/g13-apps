use crate::app::App;
use crate::app::Application;
use crate::app::APP_COUNT;
use crate::component::appbar::AppBar;
use crate::display::G13Display;
use crate::error::AppError;
use crate::style::{FILL_ON, TEXT_BOLD, TEXT_REGULAR, TITLE_BOLD};
use async_trait::async_trait;
use embedded_graphics::{
    fonts::Text, pixelcolor::BinaryColor, prelude::*, primitives::Rectangle,
    style::PrimitiveStyleBuilder,
};
use once_cell::sync::Lazy;
use std::iter::IntoIterator;
use std::marker::Unpin;
use std::str::FromStr;
use std::time::Duration;
use strum::VariantNames;
use tokio::io::AsyncWrite;
use tokio::time;

/// The G13 menu app
#[derive(Clone, Debug, Default)]
pub struct Menu {
    /// Store the index for an app
    cursor: usize,
    /// Define if the app should end/return
    end: bool,
}

#[async_trait(?Send)]
impl Application for Menu {
    async fn execute<W: Unpin + AsyncWrite>(&self, out: &mut W) -> Result<App, AppError>
    where
        W: AsyncWrite + Unpin,
    {
        let mut interval = time::interval(Duration::from_millis(100));
        let mut display = G13Display::new(out);
        let mut last_cursor: usize = usize::MAX;
        let apps: &[&'static str] = App::VARIANTS;

        // Draw the base interface
        (*MENU_INTERFACE).clone().into_iter().draw(&mut display)?;

        #[warn(clippy::while_immutable_condition)]
        while !self.end {
            // If the cursor does not change between iteration, just wait
            if last_cursor == self.cursor {
                interval.tick().await;
                continue;
            }
            last_cursor = self.cursor;

            // Draw the menu cursor to the G13 device
            for i in 0..3i32 {
                if self.cursor == 0 && i == 0 {
                    continue;
                }
                if let Some(name) = apps.get(self.cursor + (i as usize) - 1) {
                    let prefix = if i == 1 { ">" } else { " " };
                    Text::new(&format!("{} {}", prefix, name), Point::new(33, 10 + 8 * i))
                        .into_styled(*TEXT_REGULAR)
                        .draw(&mut display)?;
                }
            }

            // Flush and await
            display.flush().await?;
            interval.tick().await;
        }

        // In case, an app is selected, we ask for run it.
        Ok(App::from_str(apps[self.cursor])?)
    }

    /// Represents the selection button
    async fn button_l1(&mut self) -> Result<(), AppError> {
        self.end = true;
        Ok(())
    }

    /// Not used
    async fn button_l2(&mut self) -> Result<(), AppError> {
        Ok(())
    }

    /// Represents the previous button
    async fn button_l3(&mut self) -> Result<(), AppError> {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
        Ok(())
    }

    /// Represents the next button
    async fn button_l4(&mut self) -> Result<(), AppError> {
        if self.cursor < APP_COUNT - 1 {
            self.cursor += 1;
        }
        Ok(())
    }

    /// Represents the "return" button (it reset the cursor)
    async fn button_bd(&mut self) -> Result<(), AppError> {
        self.cursor = 0;
        Ok(())
    }
}

// The static part of the menu interface
static MENU_INTERFACE: Lazy<Vec<Pixel<BinaryColor>>> = Lazy::new(|| {
    // Draw the image container
    let img_placeholder = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .fill_color(BinaryColor::Off)
        .build();
    let image = Rectangle::new(Point::new(0, 0), Point::new(32, 32))
        .into_styled(img_placeholder)
        .into_iter();

    let appbar = AppBar::new("\u{2195} Menu", Point::new(33, 0), Point::new(159, 8)).into_iter();

    // Draw the button info
    let button1 = Text::new("OK", Point::new(40 / 2 - 12, 36))
        .into_styled(*TEXT_BOLD)
        .into_iter();
    let button3 = Text::new("\u{25B2}", Point::new(2 * 40 + 40 / 2 - 4, 36))
        .into_styled(*TEXT_BOLD)
        .into_iter();
    let button4 = Text::new("\u{25BC}", Point::new(3 * 40 + 40 / 2 - 4, 36))
        .into_styled(*TEXT_BOLD)
        .into_iter();
    image
        .chain(appbar)
        .chain(button1)
        .chain(button3)
        .chain(button4)
        .collect()
});
