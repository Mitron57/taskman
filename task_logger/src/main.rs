mod application;
mod domain;
mod handlers;
mod infrastructure;

use crate::application::{Logger, State};
use crate::handlers::poll_fs;
use crate::infrastructure::FileSystem;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    if dotenvy::dotenv().is_err() {
        println!("Cannot read .env file, trying to use manually specified env variables...");
    }
    let processor_dir = std::env::var("PROCESSOR_DIR")?;
    let logger_dir = std::env::var("LOGGER_DIR")?;
    let logger = Arc::new(Logger);
    let fs = Arc::new(FileSystem::new(logger_dir)?);
    let state = Arc::new(State { logger, fs });
    poll_fs(state, &processor_dir).await?;
    Ok(())
}
