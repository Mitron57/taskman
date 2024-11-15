use std::error::Error;
use axum::async_trait;
use crate::domain::interfaces;
use crate::domain::models::Task;

pub struct Processor;

#[async_trait]
impl interfaces::Processor for Processor {
    type Error = Box<dyn Error + Send + Sync>;

    async fn process(&self, mut task: Task) -> Result<Task, Self::Error> {
        task.content = task.content.chars().rev().collect();
        Ok(task)
    }
}