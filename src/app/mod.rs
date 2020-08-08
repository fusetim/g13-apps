use crate::error::AppError;
use async_trait::async_trait;
#[cfg(feature = "clock")]
use clock::Clock;
use enum_dispatch::enum_dispatch;
#[cfg(feature = "hello")]
use hello::Hello;
use menu::Menu;
use std::marker::Unpin;
use strum_macros::{EnumCount, EnumString, EnumVariantNames};
use tokio::io::AsyncWrite;

#[cfg(feature = "clock")]
mod clock;
pub mod error;
#[cfg(feature = "hello")]
mod hello;
mod menu;

/// List of apps hidden from the menu.
/// Needed by error app
pub const HIDDEN_APPS: &[&str] = &["error_app"];

/// Listing of all implemented applications.
/// Probably some of them will be activated only with certain features.
/// All the applications listed here must have in field, their structure which inherits the App trait.
#[enum_dispatch(Application)]
#[derive(EnumString, EnumVariantNames, EnumCount, Debug)]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum App {
    Menu,
    ErrorApp(error::Error),
    #[cfg(feature = "hello")]
    Hello,
    #[cfg(feature = "clock")]
    Clock,
}

/// Trait with all interactions between the AppManager and the App itself
#[async_trait(?Send)]
#[enum_dispatch]
pub trait Application {
    /// The main function of the application. It should not return as long as the application is open.
    /// The application to be returned is the application that will be launched after it is closed.
    /// For example, the menu can return the selected application that will be opened. Also, an application
    /// that closes must ask for the Menu to be opened (otherwise another application).
    ///
    /// Caution, the future can be destroyed during an interaction with one of the keys.
    async fn execute<W: Unpin + AsyncWrite>(&self, out: &mut W) -> Result<App, AppError>
    where
        W: AsyncWrite + Unpin;

    /// Function called as soon as the L1 key is pressed (or a matching key defined in the configuration).
    async fn button_l1(&mut self) -> Result<(), AppError>;

    /// Function called as soon as the L2 key is pressed (or a matching key defined in the configuration).
    async fn button_l2(&mut self) -> Result<(), AppError>;

    /// Function called as soon as the L3 key is pressed (or a matching key defined in the configuration).
    async fn button_l3(&mut self) -> Result<(), AppError>;

    /// Function called as soon as the L4 key is pressed (or a matching key defined in the configuration).
    async fn button_l4(&mut self) -> Result<(), AppError>;

    /// Function called as soon as the BD key is pressed (or a matching key defined in the configuration).
    async fn button_bd(&mut self) -> Result<(), AppError>;
}
