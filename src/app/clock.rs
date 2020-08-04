use crate::app::App;
use crate::app::Application;
use crate::display::G13Display;
use crate::error::AppError;
use crate::style::{BORDER_1, TEXT_BOLD, TEXT_LIGHT, TEXT_SMALL};
use async_trait::async_trait;
use chrono::DateTime;
use chrono::Local;
use embedded_graphics::geometry::Point;
use embedded_graphics::primitives::{Circle, Line};
use embedded_graphics::{fonts::Text, pixelcolor::BinaryColor, prelude::*};
use std::marker::Unpin;
use std::str::FromStr;
use std::time::Duration;
use tokio::io::AsyncWrite;
use tokio::time;

/// The G13 Clock app
/// Analog and numeric
#[derive(Clone, Debug, Default)]
pub struct Clock {
    /// Define if the app should end/return
    end: bool,
}

#[async_trait(?Send)]
impl Application for Clock {
    async fn execute<W: Unpin + AsyncWrite>(&self, out: &mut W) -> Result<App, AppError>
    where
        W: AsyncWrite + Unpin,
    {
        let mut interval = time::interval(Duration::from_millis(500));
        let mut display = G13Display::new(out);

        // Wait until user ask for the menu app
        #[warn(clippy::while_immutable_condition)]
        while !self.end {
            // Get time and date
            let now = Local::now();
            let time = now.format("%T").to_string();
            let date = now.format("%d %B %Y").to_string();

            // Draw the analog clock
            make_analog(&now).into_iter().draw(&mut display)?;

            // Draw the numeric clock
            Text::new(&time, Point::new(50, 43 / 2 - 10))
                .into_styled(*TEXT_BOLD)
                .draw(&mut display)?;
            Text::new(&date, Point::new(50, 43 / 2 + 2))
                .into_styled(*TEXT_LIGHT)
                .draw(&mut display)?;

            // Flush and await
            display.flush().await?;
            interval.tick().await;
        }

        // Return to menu app
        Ok(App::from_str("menu")?)
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

/// Make an drawable iterator of the analog clock
pub fn make_analog(time: &DateTime<Local>) -> Vec<Pixel<BinaryColor>> {
    let center = Point::new(20, 20);

    // Making the clock face
    let circle = Circle::new(center, 20).into_styled(*BORDER_1).into_iter();
    let h12 = Text::new("12", Point::new(16, 4))
        .into_styled(*TEXT_SMALL)
        .into_iter();
    let h3 = Text::new("3", Point::new(34, 18))
        .into_styled(*TEXT_SMALL)
        .into_iter();
    let h6 = Text::new("6", Point::new(18, 34))
        .into_styled(*TEXT_SMALL)
        .into_iter();
    let h9 = Text::new("9", Point::new(3, 18))
        .into_styled(*TEXT_SMALL)
        .into_iter();

    // Making the hour hand
    let cos_hour: f64 = (time.format("%I").to_string().parse::<f64>().unwrap() * 30.0 - 90.0)
        .to_radians()
        .cos();
    let sin_hour: f64 = (time.format("%I").to_string().parse::<f64>().unwrap() * 30.0 - 90.0)
        .to_radians()
        .sin();
    let hour_hand = Line::new(
        center,
        Point::new(
            (cos_hour * 10.0 + 20.0).floor() as i32,
            (sin_hour * 10.0 + 20.0).floor() as i32,
        ),
    )
    .into_styled(*BORDER_1)
    .into_iter();

    // Making the minute hand
    let cos_min: f64 = (time.format("%M").to_string().parse::<f64>().unwrap() * 6.0 - 90.0)
        .to_radians()
        .cos();
    let sin_min: f64 = (time.format("%M").to_string().parse::<f64>().unwrap() * 6.0 - 90.0)
        .to_radians()
        .sin();
    let min_hand = Line::new(
        center,
        Point::new(
            (cos_min * 15.0 + 20.0).floor() as i32,
            (sin_min * 15.0 + 20.0).floor() as i32,
        ),
    )
    .into_styled(*BORDER_1)
    .into_iter();

    circle
        .chain(h12)
        .chain(h3)
        .chain(h6)
        .chain(h9)
        .chain(hour_hand)
        .chain(min_hand)
        .collect()
}
