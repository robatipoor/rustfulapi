use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct WorkerConfig {
  pub failed_task_delay: u64,
}
