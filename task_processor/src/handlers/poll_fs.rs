use crate::application::State;
use crate::domain::models::Task;
use notify::{RecursiveMode, Watcher};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc::unbounded_channel;

pub async fn poll_fs(
    state: Arc<State>,
    creator_dir: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (sender, mut receiver) = unbounded_channel();
    let mut watcher = notify::recommended_watcher(move |event| {
        let _ = sender.send(event);
    })?;
    watcher.watch(creator_dir.as_ref(), RecursiveMode::NonRecursive)?;
    for event in receiver.recv().await.into_iter().flatten() {
        if event.kind.is_create() {
            let content = std::fs::read_to_string(&event.paths[0])?;
            let task: Task = serde_json::from_str(&content)?;
            let processed = state.processor.process(task).await?;
            state.fs.add(processed).await?;
        }
    }
    Ok(())
}
