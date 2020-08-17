use thiserror::Error;
use mpris::FindingError;
use mpris::DBusError;

/// Represents all errors that can be thrown by the music app
#[derive(Error, Debug)]
pub enum MusicError {
    /// Represents an error caused by a an underlined DBus error
    #[error("DBus error occured. reason: {0}")]
    DBusError(String),

    /// Represents an error while looking for music players
    #[error("finding players failed. reason: {0}")]
    FindingError(String),

    /// Represents a failed command
    #[error("command {0} failed. Maybe the player does not support DBus command.")]
    CommandError (super::Command, #[source] Box<MusicError>),

    /// As named, it represents an unknown error
    #[error("unknown music app error")]
    Unknown,
}

impl From<FindingError> for MusicError {
    fn from(err: FindingError) -> Self {
        Self::FindingError(err.to_string())
    }
}

impl From<DBusError> for MusicError {
    fn from(err: DBusError) -> Self {
        Self::DBusError(err.to_string())
    }
}

impl From<MusicError> for crate::error::AppError {
    fn from(err: MusicError) -> Self {
        crate::error::AppError::MusicError(err)
    }
}