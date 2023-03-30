use actix_web::web;
use configure::worker::WorkerConfig;
use futures::future::join_all;
use state::{
  worker::{AppTask, TaskStatus},
  AppState,
};
use std::time::Duration;
use tokio::task::JoinHandle;
use tracing::{error, info};

pub fn start(state: web::Data<AppState>) -> JoinHandle<std::io::Result<()>> {
  tokio::task::spawn(worker(state))
}

async fn worker(_state: web::Data<AppState>) -> std::io::Result<()> {
  let jhs: Vec<JoinHandle<()>> = vec![];
  join_all(jhs).await;
  Ok(())
}

fn _doing_job<T: AppTask + 'static + Send>(config: WorkerConfig, task: T) -> JoinHandle<()> {
  tokio::task::spawn(async move {
    info!("*** start task: {} ***", T::NAME);
    loop {
      match task.run().await {
        Ok(r) => match r.status {
          TaskStatus::Completed => {
            info!("job: {} complete", T::NAME);
            tokio::time::sleep(Duration::from_secs(r.delay)).await
          }
          TaskStatus::QueueEmpty => {
            info!("job: {} queue is empty", T::NAME);
            tokio::time::sleep(Duration::from_secs(r.delay)).await
          }
        },
        Err(e) => {
          error!("job: {} faild error message: {e:?}", T::NAME);
          tokio::time::sleep(Duration::from_secs(config.failed_task_delay)).await
        }
      }
    }
  })
}
