use crate::domain::interfaces;
use crate::domain::interfaces::FileSystem;
use crate::domain::models::Task;
use std::error::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use axum::async_trait;

#[derive(Default)]
pub struct Creator {
    id_counter: AtomicUsize,
}

#[async_trait]
impl interfaces::Creator for Creator {
    type Error = Box<dyn Error + Send + Sync>;
    type FileSystem = Arc<dyn FileSystem<Error = Self::Error> + Send + Sync>;

    async fn create_task(&self, content: String, fs: Self::FileSystem) -> Result<(), Self::Error> {
        let id = self.id_counter.fetch_add(1, Ordering::AcqRel);
        let task = Task { id, content };
        fs.add(task).await?;
        Ok(())
    }
}
