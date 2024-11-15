use std::error;
use std::sync::Arc;
use crate::domain::interfaces;

type Error = Box<dyn error::Error + Send + Sync>;
type FileSystem = Arc<dyn interfaces::FileSystem<Error = Error> + Send + Sync>;

type Creator = Arc<dyn interfaces::Creator<Error = Error, FileSystem = FileSystem> + Send + Sync>;

pub struct AppState {
    pub creator: Creator,
    pub fs: FileSystem,
}
