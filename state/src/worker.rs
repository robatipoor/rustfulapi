use crate::AppState;
use actix_web::web;
use async_trait::async_trait;
use error::TaskError;

#[derive(Debug)]
pub struct TaskResult {
  pub delay: u64,
  pub status: TaskStatus,
}

#[derive(Debug)]
pub enum TaskStatus {
  Completed,
  QueueEmpty,
}

#[async_trait]
pub trait AppTask: Send + Sync {
  const NAME: &'static str;
  fn new(state: web::Data<AppState>) -> Self;
  async fn run(&self) -> Result<TaskResult, TaskError>;
}
