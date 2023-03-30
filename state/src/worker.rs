use crate::AppState;
use actix_web::web;
use async_trait::async_trait;

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

#[derive(Debug)]
pub enum TaskError {}

#[async_trait]
pub trait AppTask: Send + Sync {
  const NAME: &'static str;
  fn new(state: web::Data<AppState>) -> Self;
  async fn run(&self) -> Result<TaskResult, TaskError>;
}
