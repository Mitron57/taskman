mod application;
mod domain;
mod handlers;
mod infrastructure;

use crate::application::{AppState, Creator};
use crate::handlers::create_task;
use crate::infrastructure::FileSystem;
use axum::routing::post;
use axum::Router;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    if dotenvy::dotenv().is_err() {
        println!("Cannot read .env file, trying to use manually specified env variables...");
    }
    let host = std::env::var("HOSTADDR")?;
    let creator_dir = std::env::var("CREATOR_DIR")?;
    let creator = Arc::new(Creator::default());
    let fs = Arc::new(FileSystem::new(creator_dir)?);
    let state = Arc::new(AppState { creator, fs });
    let router = Router::new()
        .route("/create", post(create_task))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind(&host).await?;
    println!("Listening on: {}", host);
    axum::serve(listener, router).await?;
    Ok(())
}