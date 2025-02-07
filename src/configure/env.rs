use std::str::FromStr;

use super::Profile;

pub fn get_env_source(prefix: &str) -> config::Environment {
  config::Environment::with_prefix(prefix)
    .prefix_separator("__")
    .separator("__")
}

pub fn get_profile() -> Result<Profile, String> {
  std::env::var("APP_PROFILE")
    .map(|env| Profile::from_str(&env))
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())
}
