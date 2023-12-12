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
          sender
            .send(e)
            .await
            .unwrap_or_else(|_| unreachable!("This channel never closed."));
        } else {
          error!("A task failed: {e}.");
        }
      }
    });
  }
  match receiver.recv().await {
    Some(err) => Err(err),
    None => unreachable!("This channel never closed."),
  }
}
