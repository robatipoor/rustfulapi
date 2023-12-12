pub use redis::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
  pub username: String,
  pub password: String,
  pub port: u16,
  pub host: String,
  pub database_name: String,
}

impl RedisConfig {
  pub fn get_url(&self) -> String {
    format!(
      "redis://{username}:{password}@{host}:{port}/{database_name}",
      username = self.username,
      password = self.password,
      host = self.host,
      port = self.port,
      database_name = self.database_name
    )
  }
}
