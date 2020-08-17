use crate::app::App;
use crate::app::Application;
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
use tokio::io::AsyncWrite;
use tokio::time;

/// A simple hello world app
#[derive(Clone, Debug, Default)]
pub struct Hello {
    /// Define if the app should end/return
    end: bool,
}

#[async_trait(?Send)]
impl Application for Hello {
    async fn execute<W: Unpin + AsyncWrite>(&mut self, out: &mut W) -> Result<App, AppError>
    where
        W: AsyncWrite + Unpin,
    {
        let mut interval = time::interval(Duration::from_millis(500));
        let mut display = G13Display::new(out);

        // Create a Textstyle for our hello world text
        let style = TextStyleBuilder::new(Font6x8)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build();

        // Draw the app (the hello world text) and flush
        Text::new("Hello, world!", Point::new(5, 43 / 2 - 3))
            .into_styled(style)
            .draw(&mut display)?;
        display.flush().await?;

        // Wait until user ask for the menu app
        #[warn(clippy::while_immutable_condition)]
        while !self.end {
            interval.tick().await;
        }

        // Return to menu app
        Ok(App::Menu(Default::default()))
    }

    /// Not used
    async fn button_l1(&mut self) -> Result<(), AppError> {
        Ok(())
    }

    /// Not used
    async fn button_l2(&mut self) -> Result<(), AppError> {
        Ok(())
    }

    /// Not used
    async fn button_l3(&mut self) -> Result<(), AppError> {
        Ok(())
    }

    /// Not used
    async fn button_l4(&mut self) -> Result<(), AppError> {
        Ok(())
    }

    /// Represents the return button (go to menu)
    async fn button_bd(&mut self) -> Result<(), AppError> {
        self.end = true;
        Ok(())
    }
}
