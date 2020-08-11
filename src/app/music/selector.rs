use super::MusicPlayer;
use crate::app::App;
use crate::app::Application;
use crate::component::{AppBar, Button, ButtonBar, List};
use crate::display::G13Display;
use crate::error::AppError;
use async_trait::async_trait;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use mpris::Player;
use mpris::PlayerFinder;
use once_cell::sync::Lazy;
use std::time::Duration;
use tokio::io::AsyncWrite;
use tokio::time;

/// Music Selector is an app to select the wanted player to show.
#[derive(Clone, Debug, Default)]
pub struct MusicSelector {
    end: bool,
    return_menu: bool,
    list: Option<List>,
}

#[async_trait(?Send)]
impl Application for MusicSelector {
    async fn execute<W: Unpin + AsyncWrite>(&mut self, out: &mut W) -> Result<App, AppError>
    where
        W: AsyncWrite + Unpin,
    {
        // Init the list if not
        if self.list.is_none() {
            self.init()?;
        }
        let mut interval = time::interval(Duration::from_millis(100));
        let mut display = G13Display::new(out);
        let mut last_cursor: usize = usize::MAX;
        let list = self.list.as_ref().unwrap();

        // Draw the base interface
        (*SELECTOR_INTERFACE)
            .clone()
            .into_iter()
            .draw(&mut display)?;

        while !self.end {
            // Tick and wait for new cursor
            interval.tick().await;
            if last_cursor == list.get_cursor() {
                continue;
            }
            last_cursor = list.get_cursor();

            // Draw and flush
            list.draw_default(&mut display)?;
            display.flush().await?;
        }

        if self.return_menu {
            Ok(App::Menu(Default::default()))
        } else {
            Ok(App::MusicPlayer(MusicPlayer::new(list.get_current())))
        }
    }

    // Selection button
    async fn button_l1(&mut self) -> Result<(), AppError> {
        self.end = true;
        Ok(())
    }

    // Not used
    async fn button_l2(&mut self) -> Result<(), AppError> {
        Ok(())
    }

    // Previous button
    async fn button_l3(&mut self) -> Result<(), AppError> {
        self.list
            .as_mut()
            .ok_or(AppError::BadInitialization)?
            .previous();
        Ok(())
    }

    // Next button
    async fn button_l4(&mut self) -> Result<(), AppError> {
        self.list
            .as_mut()
            .ok_or(AppError::BadInitialization)?
            .next();
        Ok(())
    }

    // Exit and return to menu
    async fn button_bd(&mut self) -> Result<(), AppError> {
        self.end = true;
        self.return_menu = true;
        Ok(())
    }
}

impl MusicSelector {
    /// Init the list component
    fn init(&mut self) -> Result<(), AppError> {
        // Get a player finder
        let finder = PlayerFinder::new().map_err(|_| AppError::DBusError)?;
        // get the players list
        let players: Vec<Player<'_>> = finder
            .find_all()
            .map_err(|_| AppError::SourceFindingError)?;
        // get their names
        let names: Vec<String> = players.iter().map(|p| p.identity().to_owned()).collect();
        // create the list
        self.list = Some(List::new(names));
        Ok(())
    }
}

// The static part of the selector interface
static SELECTOR_INTERFACE: Lazy<Vec<Pixel<BinaryColor>>> = Lazy::new(|| {
    // Draw the app bar
    let appbar = AppBar::new("Select a player:", Point::zero(), Point::new(160, 8)).into_iter();

    // Draw the button info
    let mut buttonbar: ButtonBar = Default::default();
    buttonbar.set_button1(Some(Button::from_str("OK")));
    buttonbar.set_button3(Some(Button::from_str("\u{25B2}")));
    buttonbar.set_button4(Some(Button::from_str("\u{25BC}")));

    appbar.chain(buttonbar.into_iter()).collect()
});
