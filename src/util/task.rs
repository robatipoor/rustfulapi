use tracing::error;

use crate::error::{AppError, AppResult};

/// If a task is fail fast after encounter an error node goes down.
pub type IsFailFast = bool;
pub type Task = (IsFailFast, futures::future::BoxFuture<'static, AppResult>);

pub async fn join_all(tasks: Vec<Task>) -> AppResult {
  let (sender, mut receiver) = tokio::sync::mpsc::channel::<AppError>(1);
  for (is_fail_fast, task) in tasks {
    let sender = if is_fail_fast {
      Some(sender.clone())
    } else {
      None
    };
    tokio::spawn(async {
      if let Err(e) = task.await {
        if let Some(sender) = sender {
          let _ = sender.send(e).await;
        } else {
          error!("A task failed: {e}.");
        }
      }
    });
  }

  // Explicitly drop the sender to close the channel.
  drop(sender);

  // Return Ok(()) if all futures are completed without error.
  match receiver.recv().await {
    Some(err) => Err(err),
    None => Ok(()),
  }
}
