use thiserror::Error;
use tokio::io;

/// The master error for this binary
#[derive(Error, Debug)]
pub enum Error {
    /// Represents an error occured by one of the g13 apps
    #[error(transparent)]
    AppError(#[from] crate::error::AppError),

    /// Represents an error dues to the named pipes
    #[error("bad pipe error")]
    BadPipeError(#[from] io::Error),

    /// As named, it represents an unknown error
    #[error("unknown error")]
    Unknown,
}

/// All errors that can be produced by a g13 app
#[derive(Error, Debug)]
pub enum AppError {
    /// Represents an parsing error for an App.
    /// Its names is unknown in the [crate::app::App] enum
    #[error("app named {name} does not exist")]
    UnknownApp{
        name: String,
        #[source] 
        source: strum::ParseError
    },

    /// Represents an error while using a badly initilized component
    #[error("component badly initialized")]
    BadInitialization,

    /// Represents a special Error from music app
    #[cfg(feature = "music")]
    #[error(transparent)]
    MusicError(crate::app::MusicError),

    /// Represents an error caused by the G13 display
    #[error(transparent)]
    DisplayError(#[from] crate::error::DisplayError),

    /// As named, it represents an unknown error
    #[error("unknown app error")]
    Unknown,
}

/// All errors that can be produced by the g13 display
#[derive(Error, Debug)]
pub enum DisplayError {
    /// Represents an error occured by writing in the G13 named pipes
    #[error("display and/or g13 service disconnected")]
    Disconnect(#[from] io::Error),

    /// As named, it represents an unknown error
    #[error("unknown display error")]
    Unknown,
}
