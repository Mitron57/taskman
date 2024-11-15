use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Content {
    pub content: String
}