use thiserror::Error;
use tokio::io;

#[derive(Error, Debug)]
pub enum DisplayError {
    #[error("display and/or service disconnected")]
    Disconnect(#[from] io::Error),

    #[error("unknown display error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("unknown app error")]
    Unknown,
}
