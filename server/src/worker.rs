use actix_web::web;
use configure::worker::WorkerConfig;
use error::TaskError;
use futures::future::join_all;
use state::{
  worker::{AppTask, TaskResult, TaskStatus},
  AppState,
};
use std::time::Duration;
use tokio::task::JoinHandle;
use tracing::{error, info};

pub fn spawn(state: web::Data<AppState>) -> JoinHandle<std::io::Result<()>> {
  tokio::task::spawn(worker(state))
}

async fn worker(state: web::Data<AppState>) -> std::io::Result<()> {
  let jhs: Vec<JoinHandle<()>> = vec![doing_job(state.config.worker.clone(), SampleTask)];
  join_all(jhs).await;
  Ok(())
}

fn doing_job<T: AppTask + 'static + Send>(config: WorkerConfig, task: T) -> JoinHandle<()> {
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
          error!("job: {} failed error message: {e:?}", T::NAME);
          tokio::time::sleep(Duration::from_secs(config.failed_task_delay)).await
        }
      }
    }
  })
}

pub struct SampleTask;

#[async_trait::async_trait]
impl AppTask for SampleTask {
  const NAME: &'static str = "SAMPLE_TASK";

  fn new(_state: web::Data<AppState>) -> Self {
    Self
  }

  async fn run(&self) -> Result<TaskResult, TaskError> {
    Ok(TaskResult {
      delay: 100000,
      status: TaskStatus::Completed,
    })
  }
}
