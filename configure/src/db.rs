use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

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

  pub fn get_connection_options(&self) -> PgConnectOptions {
    PgConnectOptions::new()
      .host(&self.host)
      .username(&self.username)
      .password(&self.password)
      .port(self.port)
      .database(&self.database_name)
      // TODO add postgres ssl config
      .ssl_mode(PgSslMode::Prefer)
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
