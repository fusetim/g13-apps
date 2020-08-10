use super::Song;
use crate::app::App;
use crate::app::Application;
use crate::component::{AppBar, Button, ButtonBar};
use crate::display::G13Display;
use crate::error::AppError;
use crate::style::{FILL_OFF, FILL_ON, TEXT_LIGHT, TEXT_SMALL};
use async_trait::async_trait;
use embedded_graphics::egrectangle;
use embedded_graphics::geometry::Point;
use embedded_graphics::{fonts::Text, pixelcolor::BinaryColor, prelude::*};
use mpris::Player;
use mpris::PlayerFinder;
use once_cell::sync::Lazy;
use std::time::Duration;
use tokio::io::AsyncWrite;
use tokio::time;

#[derive(Clone, Debug)]
enum Command {
    PlayPause,
    Stop,
    Previous,
    Next,
}

/// Music Selector is ann app to select the wanted player to show.
#[derive(Clone, Debug, Default)]
pub struct MusicPlayer {
    end: bool,
    player_name: String,
    commands: Vec<Command>,
}

#[async_trait(?Send)]
impl Application for MusicPlayer {
    async fn execute<W: Unpin + AsyncWrite>(&mut self, out: &mut W) -> Result<App, AppError>
    where
        W: AsyncWrite + Unpin,
    {
        let mut interval = time::interval(Duration::from_millis(100));
        let mut display = G13Display::new(out);
        // Get a player finder
        let finder = PlayerFinder::new().map_err(|_| AppError::DBusError)?;
        // get the players list
        let players: Vec<Player<'_>> = finder
            .find_all()
            .map_err(|_| AppError::SourceFindingError)?;
        // get the wanted player
        let player: Player = players
            .into_iter()
            .find(|player| player.identity() == &self.player_name)
            .ok_or(AppError::BadInitialization)?;

        // Draw the base interface
        (*PLAYER_INTERFACE).clone().into_iter().draw(&mut display)?;

        let mut last_song: Option<Song> = None;
        while !self.end {
            // Tick
            interval.tick().await;

            // execute command in the queue
            while let Some(cmd) = self.commands.pop() {
                match cmd {
                    Command::PlayPause => player
                        .checked_play_pause()
                        .map_err(|_| AppError::DBusError)?,
                    Command::Stop => player.checked_stop().map_err(|_| AppError::DBusError)?,
                    Command::Previous => {
                        player.checked_previous().map_err(|_| AppError::DBusError)?
                    }
                    Command::Next => player.checked_next().map_err(|_| AppError::DBusError)?,
                };
            }

            // get the current song
            let song = Song::from(player.get_metadata().map_err(|_| AppError::DBusError)?);

            // continue until song changed
            if Some(&song) == last_song.as_ref() {
                continue;
            }
            last_song = Some(song.clone());

            // Clear the last song
            egrectangle!(
                top_left = Point::new(0, 10),
                bottom_right = Point::new(160, 34),
                style = *FILL_OFF,
            )
            .draw(&mut display);

            // Print the song title
            let mut offset = 10;
            let mut lines = song.title.clone();
            for _ in 1..2 {
                // Get the number of chars to print
                let chars = if lines.len() < 26 { lines.len() } else { 20 };
                // Get the chars and print
                let line: String = lines.drain(..chars).collect();
                Text::new(&line, Point::new(0, offset))
                    .into_styled(*TEXT_LIGHT)
                    .draw(&mut display)?;

                offset += 8;
            }

            // Print the artist name
            Text::new("by", Point::new(0, offset + 2))
                .into_styled(*TEXT_SMALL)
                .draw(&mut display)?;
            Text::new(&song.artist, Point::new(16, offset))
                .into_styled(*TEXT_LIGHT)
                .draw(&mut display)?;

            display.flush().await?;
        }

        Ok(App::Menu(Default::default()))
    }

    // Selection button
    async fn button_l1(&mut self) -> Result<(), AppError> {
        self.commands.push(Command::PlayPause);
        Ok(())
    }

    // Stop button
    async fn button_l2(&mut self) -> Result<(), AppError> {
        self.commands.push(Command::Stop);
        Ok(())
    }

    // Previous button
    async fn button_l3(&mut self) -> Result<(), AppError> {
        self.commands.push(Command::Previous);
        Ok(())
    }

    // Next button
    async fn button_l4(&mut self) -> Result<(), AppError> {
        self.commands.push(Command::Next);
        Ok(())
    }

    // Exit and return to menu
    async fn button_bd(&mut self) -> Result<(), AppError> {
        self.end = true;
        Ok(())
    }
}

impl MusicPlayer {
    /// Create a MusicPlayer using the given MPRIS Player name
    pub fn new<S: Into<String>>(player_name: S) -> Self {
        Self {
            end: false,
            player_name: player_name.into(),
            commands: Vec::new(),
        }
    }
}

// The static part of the selector interface
static PLAYER_INTERFACE: Lazy<Vec<Pixel<BinaryColor>>> = Lazy::new(|| {
    // Draw the app bar
    let appbar = AppBar::new("\u{266B} Playing:", Point::zero(), Point::new(160, 8)).into_iter();

    // Draw the button info
    let mut buttonbar: ButtonBar = Default::default();
    buttonbar.set_button1(Some(Button::from_str("\u{25BA}")));
    // The stop icon
    let button2 = egrectangle!(
        top_left = Point::zero(),
        bottom_right = Point::new(6, 6),
        style = *FILL_ON,
    );
    buttonbar.set_button2(Some(Button::from_drawable(
        &button2,
        button2.top_left(),
        button2.bottom_right(),
    )));
    buttonbar.set_button3(Some(Button::from_str("\u{25C4}\u{25C4}")));
    buttonbar.set_button4(Some(Button::from_str("\u{25BA}\u{25BA}")));

    appbar.chain(buttonbar.into_iter()).collect()
});
