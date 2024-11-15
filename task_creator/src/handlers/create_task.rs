use crate::application::AppState;
use crate::handlers::dto::Content;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, Json};
use serde_json::json;
use std::sync::Arc;

#[debug_handler]
pub async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(content): Json<Content>,
) -> impl IntoResponse {
    if let Err(err) = state.creator.create_task(content.content, state.fs.clone()).await {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": err.to_string() })),
        )
    } else {
        (StatusCode::CREATED, Json(json!({})))
    }
}
