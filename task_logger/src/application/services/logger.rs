use std::error::Error;
use axum::async_trait;
use crate::domain::interfaces;
use crate::domain::models::Log;

pub struct Logger;

#[async_trait]
impl interfaces::Logger for Logger {
    type Error = Box<dyn Error + Send + Sync>;

    async fn log(&self, id: usize, message: String) -> Result<Log, Self::Error> {
        Ok(Log{id, message})
    }
}