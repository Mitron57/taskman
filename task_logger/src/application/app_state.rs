use std::error;
use std::sync::Arc;
use crate::domain::interfaces;

type Error = Box<dyn error::Error + Sync + Send>;
type Logger = dyn interfaces::Logger<Error = Error> + Send + Sync;
type FileSystem = dyn interfaces::FileSystem<Error = Error> + Send + Sync;

pub struct State {
    pub logger: Arc<Logger>,
    pub fs: Arc<FileSystem>,
}