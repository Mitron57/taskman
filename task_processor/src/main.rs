mod application;
mod domain;
mod handlers;
mod infrastructure;

use crate::application::{Processor, State};
use crate::handlers::poll_fs;
use crate::infrastructure::FileSystem;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if dotenvy::dotenv().is_err() {
        println!("Cannot read .env file, trying to use manually specified env variables...");
    }
    let creator_dir = std::env::var("CREATOR_DIR")?;
    let processor_dir = std::env::var("PROCESSOR_DIR")?;
    let processor = Arc::new(Processor);
    let fs = Arc::new(FileSystem::new(processor_dir)?);
    let state = Arc::new(State { processor, fs });
    poll_fs(state, &creator_dir).await?;
    Ok(())
}
