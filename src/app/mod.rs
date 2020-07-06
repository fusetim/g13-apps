use crate::error::AppError;
use async_trait::async_trait;
use menu::Menu;
use tokio::io::AsyncWrite;

mod menu;

/// Listing of all implemented applications.
/// Probably some of them will be activated only with certain features.
/// All the applications listed here must have in field, their structure which inherits the App trait.
pub enum Apps {
    Menu(Menu),
}

impl From<usize> for Apps {
    /// Create an Application instance with its associated id number
    /// Mainly used by other applications to target the application to be opened when they are closed
    fn from(index: usize) -> Self {
        match index {
            menu if menu == Apps::Menu as usize => Apps::Menu(Menu::default()),
            _ => Apps::Menu(Menu::default()),
        }
    }
}

/// Trait with all interactions between the AppManager and the App itself
#[async_trait()]
pub trait App {
    /// The main function of the application. It should not return as long as the application is open.
    /// The application to be returned is the application that will be launched after it is closed.
    /// For example, the menu can return the selected application that will be opened. Also, an application
    /// that closes must ask for the Menu to be opened (otherwise another application).
    ///
    /// Caution, the future can be destroyed during an interaction with one of the keys.
    async fn execute<W: Send + AsyncWrite>(&self, out: &mut W) -> Result<Apps, AppError>
    where
        W: AsyncWrite;

    /// Function called as soon as the L1 key is pressed (or a match defined in the configuration).
    async fn button_l1(&self) -> Result<(), AppError>;

    /// Function called as soon as the L2 key is pressed (or a match defined in the configuration).
    async fn button_l2(&self) -> Result<(), AppError>;

    /// Function called as soon as the L3 key is pressed (or a match defined in the configuration).
    async fn button_l3(&self) -> Result<(), AppError>;

    /// Function called as soon as the L4 key is pressed (or a match defined in the configuration).
    async fn button_l4(&self) -> Result<(), AppError>;

    /// Function called as soon as the BD key is pressed (or a match defined in the configuration).
    async fn button_bd(&self) -> Result<(), AppError>;
}
