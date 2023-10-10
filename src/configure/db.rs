use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
  pub username: String,
  pub password: String,
  pub port: u16,
  pub host: String,
  pub max_connections: u32,
  pub database_name: String,
}

impl DatabaseConfig {
  pub fn get_url(&self) -> String {
    Self::create_url(
      &self.username,
      &self.password,
      &self.host,
      self.port,
      &self.database_name,
    )
  }

  pub fn create_url(
    username: &str,
    password: &str,
    host: &str,
    port: u16,
    database_name: &str,
  ) -> String {
    format!("postgres://{username}:{password}@{host}:{port}/{database_name}")
  }
}
