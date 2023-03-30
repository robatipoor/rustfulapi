use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HttpClientConfig {
  pub timeout: u64,
}
