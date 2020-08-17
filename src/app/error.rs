use crate::app::App;
use crate::app::Application;
use crate::component::AppBar;
use crate::display::G13Display;
use crate::error::AppError;
use crate::style::{TEXT_LIGHT, TEXT_SMALL};
use async_trait::async_trait;
use embedded_graphics::{fonts::Text, prelude::*};
use std::marker::Unpin;
use std::str::FromStr;
use std::time::Duration;
use tokio::io::AsyncWrite;
use tokio::time;

/// An error app
///
/// Will be show when an app return an AppError
#[derive(Debug)]
pub struct Error {
    /// Define if the app should end/return
    end: bool,
    error: String,
    return_to: String,
}

impl Default for Error {
    // By default, no error has occured, so it's an error to open this app.
    fn default() -> Self {
        Self {
            end: true,
            error: "Oops\nNo error has occured\nSorry.".to_owned(),
            return_to: "menu".to_owned(),
        }
    }
}

impl Error {
    /// Create an Error app with the given error
    pub fn new<S: std::fmt::Display>(error: S) -> Self {
        Self {
            end: false,
            error: error.to_string(),
            return_to: "menu".to_owned(),
        }
    }

    /// Create an Error app with the given error and return app.
    /// The return app should be a valid string in the crate::app::App enum.
    pub fn with_return<E: std::fmt::Display, S: Into<String>>(error: E, return_to: S) -> Self {
        Self {
            end: false,
            error: error.to_string(),
            return_to: return_to.into(),
        }
    }
}

#[async_trait(?Send)]
impl Application for Error {
    async fn execute<W: Unpin + AsyncWrite>(&mut self, out: &mut W) -> Result<App, AppError>
    where
        W: AsyncWrite + Unpin,
    {
        let mut interval = time::interval(Duration::from_millis(500));
        let mut display = G13Display::new(out);

        // Setup the appbar
        AppBar::new("An error occured:", Point::zero(), Point::new(160, 10)).draw(&mut display)?;

        // Print the lines 1-3 to the app
        let mut offset = 12;
        let mut lines = self.error.clone();
        for _ in 1..=3 {
            // Get the number of chars to print
            let chars = if lines.len() < 26 { lines.len() } else { 26 };
            // Get the chars and print
            let line: String = lines.drain(..chars).collect();
            Text::new(&line, Point::new(0, offset))
                .into_styled(*TEXT_LIGHT)
                .draw(&mut display)?;

            offset += 8;
        }

        // Added a more info in the buttom line
        Text::new(
            "More info in logs... Use BD to continue...",
            Point::new(0, 38),
        )
        .into_styled(*TEXT_SMALL)
        .draw(&mut display)?;

        display.flush().await?;

        // Wait until user ask to continue
        #[warn(clippy::while_immutable_condition)]
        while !self.end {
            interval.tick().await;
        }

        match App::from_str(&self.return_to) {
            Ok(app) => Ok(app),
            Err(err) => Err(AppError::UnknownApp{name: self.return_to.clone(), source: err})
        }
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
