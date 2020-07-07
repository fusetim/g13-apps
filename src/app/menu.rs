use crate::app::App;
use crate::app::Application;
use crate::app::APP_COUNT;
use crate::display::G13Display;
use crate::error::AppError;
use async_trait::async_trait;
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
};
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

        // Construct a TextStyle for the menu text - temporary
        let style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::Off)
        .background_color(BinaryColor::On)
        .build();

        #[warn(clippy::while_immutable_condition)]
        while !self.end {
            // If the cursor does not change between iteration, just wait
            if last_cursor == self.cursor {
                interval.tick().await;
                continue;
            }
            last_cursor = self.cursor;      

            // Draw the menu cursor to the G13 device
            Text::new(
                &format!("Menu; cursor: {}", self.cursor),
                Point::new(5, 43 / 2 - 3),
            )
            .into_styled(style)
            .draw(&mut display)?;

            // Flush and wait
            display.flush().await?;
            interval.tick().await;
        }

        // In case, an app is selected, we ask for run it.
        let apps: &[&'static str] = App::VARIANTS;
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
