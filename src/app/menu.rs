use crate::app::App;
use crate::app::Apps;
use crate::error::AppError;
use async_trait::async_trait;
use tokio::io::AsyncWrite;

/// The future Menu app
#[derive(Clone, Debug, Default)]
pub struct Menu {}

#[async_trait()]
impl App for Menu {
    async fn execute<W: Send + AsyncWrite>(&self, out: &mut W) -> Result<Apps, AppError>
    where
        W: AsyncWrite,
    {
        unimplemented!()
    }
    async fn button_l1(&self) -> Result<(), AppError> {
        unimplemented!()
    }
    async fn button_l2(&self) -> Result<(), AppError> {
        unimplemented!()
    }
    async fn button_l3(&self) -> Result<(), AppError> {
        unimplemented!()
    }
    async fn button_l4(&self) -> Result<(), AppError> {
        unimplemented!()
    }
    async fn button_bd(&self) -> Result<(), AppError> {
        unimplemented!()
    }
}
