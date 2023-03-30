use std::fmt;

use ::tracing::info;
use config::ConfigError;
use once_cell::sync::Lazy;
use serde::Deserialize;

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

pub static CONFIG: Lazy<AppConfig> = Lazy::new(|| AppConfig::read().unwrap());

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
      util::file::root_dir("settings").map_err(|e| ConfigError::Message(e.to_string()))?;
    let profile: Profile = std::env::var("APP_PROFILE")
      .unwrap_or_else(|_| "dev".into())
      .try_into()
      .map_err(ConfigError::Message)?;
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
    info!("success read config profile: {profile}");
    config.try_deserialize()
  }
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Profile {
  #[serde(rename = "test")]
  Test,
  #[serde(rename = "dev")]
  Dev,
  #[serde(rename = "prod")]
  Prod,
}

impl fmt::Display for Profile {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Profile::Test => write!(f, "test"),
      Profile::Dev => write!(f, "dev"),
      Profile::Prod => write!(f, "prod"),
    }
  }
}

impl TryFrom<&str> for Profile {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value.to_lowercase().as_str() {
      "test" => Ok(Self::Test),
      "dev" => Ok(Self::Dev),
      "prod" => Ok(Self::Prod),
      other => Err(format!(
        "{other} is not a supported environment. Use either `dev` or `prod` or `test`."
      )),
    }
  }
}

impl TryFrom<String> for Profile {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    Profile::try_from(&*value)
  }
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
    let profile: Profile = Profile::try_from("Dev").unwrap();
    assert_eq!(profile, Profile::Dev)
  }
}
