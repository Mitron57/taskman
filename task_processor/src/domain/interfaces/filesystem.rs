use axum::async_trait;
use crate::domain::models::Task;

#[async_trait]
pub trait FileSystem {
    type Error;
    async fn add(&self, task: Task) -> Result<(), Self::Error>;
}