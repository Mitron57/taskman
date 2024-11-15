use crate::domain::interfaces;
use std::error;
use std::sync::Arc;

type Error = Box<dyn error::Error + Sync + Send>;
type Processor = dyn interfaces::Processor<Error = Error> + Send + Sync;
type FileSystem = dyn interfaces::FileSystem<Error = Error> + Send + Sync;

pub struct State {
    pub processor: Arc<Processor>,
    pub fs: Arc<FileSystem>,
}
