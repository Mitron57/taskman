use crate::domain::interfaces;
use crate::domain::models::Task;
use axum::async_trait;
use serde_json::json;
use std::error::Error;

pub struct FileSystem {
    creator_dir: String,
}

impl FileSystem {
    pub fn new(creator_dir: String) -> Result<FileSystem, Box<dyn Error>> {
        std::fs::create_dir_all(&creator_dir)?;
        Ok(FileSystem { creator_dir })
    }
}

#[async_trait]
impl interfaces::FileSystem for FileSystem {
    type Error = Box<dyn Error + Send + Sync>;

    async fn add(&self, task: Task) -> Result<(), Self::Error> {
        let path = format!("{}/{}.task", self.creator_dir, task.id);
        tokio::fs::write(path, serde_json::to_vec(&task)?).await?;
        Ok(())
    }
}
