use axum::async_trait;
use crate::domain::models::Task;

#[async_trait]
pub trait Processor {
    type Error;
    async fn process(&self, task: Task) -> Result<Task, Self::Error>;
}