use axum::async_trait;

#[async_trait]
pub trait Creator {
    type Error;
    type FileSystem;
    async fn create_task(&self, content: String, fs: Self::FileSystem) -> Result<(), Self::Error>;
}