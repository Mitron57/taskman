use crate::domain::interfaces;
use crate::domain::models::Task;
use axum::async_trait;
use std::error;

type Error = Box<dyn error::Error + Send + Sync>;

pub struct FileSystem {
    processor_dir: String,
}

impl FileSystem {
    pub fn new(processor_dir: String) -> Result<FileSystem, Error> {
        std::fs::create_dir_all(&processor_dir)?;
        Ok(FileSystem { processor_dir })
    }
}

#[async_trait]
impl interfaces::FileSystem for FileSystem {
    type Error = Error;

    async fn add(&self, task: Task) -> Result<(), Self::Error> {
        let path = format!("{}/{}.proc", self.processor_dir, task.id);
        tokio::fs::write(path, serde_json::to_vec(&task)?).await?;
        Ok(())
    }
}
