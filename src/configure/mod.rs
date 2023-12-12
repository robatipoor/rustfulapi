use std::str::FromStr;

use ::tracing::info;
use config::ConfigError;
use serde::Deserialize;

use crate::util;

use self::{
  db::DatabaseConfig, email::EmailConfig, http::HttpClientConfig, redis::RedisConfig,
  secret::SecretConfig, sentry::SentryConfig, server::ServerConfig, worker::WorkerConfig,
};

pub mod db;
pub mod email;
pub mod http;
pub mod redis;
pub mod secret;
pub mod sentry;
pub mod server;
pub mod template;
pub mod tracing;
pub mod worker;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
  pub profile: Profile,
  pub server: ServerConfig,
  pub db: DatabaseConfig,
  pub redis: RedisConfig,
  pub email: EmailConfig,
  pub sentry: SentryConfig,
  pub secret: SecretConfig,
  pub worker: WorkerConfig,
  pub http: HttpClientConfig,
}

impl AppConfig {
  pub fn read() -> Result<Self, config::ConfigError> {
    let config_dir =
      util::dir::root_dir("settings").map_err(|e| ConfigError::Message(e.to_string()))?;
    let profile = std::env::var("APP_PROFILE")
      .map(|env| Profile::from_str(&env).map_err(|e| ConfigError::Message(e.to_string())))
      .unwrap_or_else(|_e| Ok(Profile::Dev))?;
    let profile_filename = format!("{profile}.toml");
    let config = config::Config::builder()
      .add_source(config::File::from(config_dir.join("base.toml")))
      .add_source(config::File::from(config_dir.join(profile_filename)))
      .add_source(
        config::Environment::with_prefix("APP")
          .prefix_separator("_")
          .separator("__"),
      )
      .build()?;
    info!("Successfully read config profile: {profile}.");
    config.try_deserialize()
  }
}

#[derive(
  Debug, strum::Display, strum::EnumString, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy,
)]
pub enum Profile {
  #[serde(rename = "test")]
  #[strum(serialize = "test")]
  Test,
  #[serde(rename = "dev")]
  #[strum(serialize = "dev")]
  Dev,
  #[serde(rename = "prod")]
  #[strum(serialize = "prod")]
  Prod,
}

#[cfg(test)]
mod tests {
  use std::convert::TryFrom;

  pub use super::*;

  #[test]
  pub fn test_read_app_config() {
    let _config = AppConfig::read().unwrap();
  }

  #[test]
  pub fn test_profile_to_string() {
    let profile: Profile = Profile::try_from("dev").unwrap();
    assert_eq!(profile, Profile::Dev)
  }
}
