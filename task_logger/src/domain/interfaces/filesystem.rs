use axum::async_trait;
use crate::domain::models::Log;

#[async_trait]
pub trait FileSystem {
    type Error;
    async fn add(&self, log: Log) -> Result<(), Self::Error>;
}