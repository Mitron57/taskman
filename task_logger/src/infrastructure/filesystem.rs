use std::error;
use axum::async_trait;
use crate::domain::interfaces;
use crate::domain::models::Log;

pub struct FileSystem {
    logger_dir: String,
}

type Error = Box<dyn error::Error + Send + Sync>;

impl FileSystem {
    pub fn new(logger_dir: String) -> Result<Self, Error> {
        std::fs::create_dir_all(&logger_dir)?;
        Ok(Self { logger_dir })
    }
}

#[async_trait]
impl interfaces::FileSystem for FileSystem {
    type Error = Error;

    async fn add(&self, log: Log) -> Result<(), Self::Error> {
        let path = format!("{}/{}.log", self.logger_dir, log.id);
        tokio::fs::write(&path, &serde_json::to_vec(&log)?).await?;
        Ok(())
    }
}
