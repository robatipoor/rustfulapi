use sentry::ClientInitGuard;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SentryConfig {
  key: String,
}

pub fn init(config: &SentryConfig) -> ClientInitGuard {
  sentry::init((
    config.key.clone(),
    sentry::ClientOptions {
      release: sentry::release_name!(),
      ..Default::default()
    },
  ))
}
