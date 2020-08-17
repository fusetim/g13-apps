mod player;
mod selector;
mod error;

pub use player::MusicPlayer;
pub use selector::MusicSelector;
pub use error::MusicError;

use mpris::{Metadata, MetadataValue};
use std::string::ToString;
use strum_macros::Display;

#[derive(Clone, Debug, Display)]
pub enum Command {
    PlayPause,
    Stop,
    Previous,
    Next,
}

/// Represents a Song with only the basic info : title and artists.
#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct Song {
    title: String,
    artist: String,
}

impl From<Metadata> for Song {
    fn from(meta: Metadata) -> Self {
        Song {
            title: meta.title().unwrap_or("Unknown").to_owned(),
            artist: get_artists(&meta).unwrap_or("Unknown".to_string()),
        }
    }
}

/// From metadata, extracts the artists name
fn get_artists(meta: &Metadata) -> Option<String> {
    match meta.get("xesam:artist") {
        Some(MetadataValue::String(name)) => Some(format_artists(name.to_owned())),
        Some(MetadataValue::Array(list)) => {
            let artists: Vec<String> = list
                .into_iter()
                .filter_map(|value| {
                    if let MetadataValue::String(name) = value {
                        return Some(format_artists(name.to_owned()));
                    }
                    None
                })
                .collect();
            Some(artists.join(" & "))
        }

        _ => None,
    }
}

/// Format the artists name to get a clean name
fn format_artists(artists: String) -> String {
    artists
        .replace(" - Topic", "")
        .replace("VEVO", "")
        .replace("; ", " & ")
}
