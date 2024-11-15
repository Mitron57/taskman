use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub content: String,
}