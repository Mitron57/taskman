use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Log {
    pub(crate) id: usize,
    pub(crate) message: String,
}