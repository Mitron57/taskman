use axum::async_trait;
use crate::domain::models::Log;

#[async_trait]
pub trait Logger {
    type Error;
    async fn log(&self, id: usize, message: String) -> Result<Log, Self::Error>;
}