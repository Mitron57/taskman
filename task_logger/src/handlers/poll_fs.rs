use std::error;
use std::sync::Arc;
use notify::{RecursiveMode, Watcher};
use tokio::sync::mpsc::unbounded_channel;
use crate::application::State;

type Error = Box<dyn error::Error + Send + Sync>;
pub async fn poll_fs(
    state: Arc<State>,
    processor_dir: &str,
) -> Result<(), Error> {
    let (sender, mut receiver) = unbounded_channel();
    let mut watcher = notify::recommended_watcher(move |event| {
        let _ = sender.send(event);
    })?;
    watcher.watch(processor_dir.as_ref(), RecursiveMode::NonRecursive)?;
    for event in receiver.recv().await.into_iter().flatten() {
        if event.kind.is_create() {
            let id_str = event.paths[0].to_str().unwrap().split_whitespace().last().unwrap();
            let id = id_str.strip_suffix(".proc").unwrap().parse::<usize>()?;
            let log = state.logger.log(id, format!("{id}.proc processed successfully")).await?;
            state.fs.add(log).await?;
        }
    }
    Ok(())
}
