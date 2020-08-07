use crate::app::App;
use crate::app::Application;
use crate::component::appbar::AppBar;
use crate::component::buttonbar::{Button, ButtonBar};
use crate::component::list::List;
use crate::display::G13Display;
use crate::error::AppError;
use async_trait::async_trait;
use embedded_graphics::{
    pixelcolor::BinaryColor, prelude::*, primitives::Rectangle, style::PrimitiveStyleBuilder,
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
#[derive(Clone, Debug)]
pub struct Menu {
    /// Define if the app should end/return
    end: bool,
    /// The internal list of apps component
    list: List,
}

impl Default for Menu {
    fn default() -> Self {
        let apps: &[&'static str] = App::VARIANTS;
        Self {
            end: false,
            // Build the app list
            list: List::new(apps.iter().map(|name| name.to_string()).collect()),
        }
    }
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

        // Draw the base interface
        (*MENU_INTERFACE).clone().into_iter().draw(&mut display)?;

        #[warn(clippy::while_immutable_condition)]
        while !self.end {
            // If the cursor does not change between iteration, just wait
            let list: &List = &self.list;
            let cursor = list.get_cursor();
            if last_cursor == cursor {
                interval.tick().await;
                continue;
            }
            last_cursor = cursor;

            // Draw the menu cursor to the G13 device
            list.draw_within_border(&mut display, Point::new(32, 10), Point::new(160, 35))?;

            // Flush and await
            display.flush().await?;
            interval.tick().await;
        }

        // In case, an app is selected, we ask for run it.
        Ok(App::from_str(self.list.get_current())?)
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
        self.list.previous();
        Ok(())
    }

    /// Represents the next button
    async fn button_l4(&mut self) -> Result<(), AppError> {
        self.list.next();
        Ok(())
    }

    /// Represents the "return" button (it reset the cursor)
    async fn button_bd(&mut self) -> Result<(), AppError> {
        self.list.reset();
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
    let mut buttonbar: ButtonBar = Default::default();
    buttonbar.set_button1(Some(Button::from_str("OK")));
    buttonbar.set_button3(Some(Button::from_str("\u{25B2}")));
    buttonbar.set_button4(Some(Button::from_str("\u{25BC}")));

    image.chain(appbar).chain(buttonbar.into_iter()).collect()
});
