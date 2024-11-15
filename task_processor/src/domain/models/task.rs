use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Task {
    pub id: usize,
    #[serde(rename(serialize = "processed_content"))]
    pub content: String,
}
