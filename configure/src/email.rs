use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct EmailConfig {
  pub username: String,
  pub password: String,
  pub port: u16,
  pub host: String,
}

impl EmailConfig {
  pub fn get_addr(&self) -> String {
    format!("{}:{}", self.host, self.port)
  }
}
