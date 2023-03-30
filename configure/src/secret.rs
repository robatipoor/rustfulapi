use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SecretConfig {
  pub private_access_key: String,
  pub public_access_key: String,
  pub private_refresh_key: String,
  pub public_refresh_key: String,
}

impl SecretConfig {
  pub fn read_private_access_key(&self) -> Result<String, std::io::Error> {
    fs::read_to_string(util::file::root_dir(&self.private_access_key)?)
  }

  pub fn read_public_access_key(&self) -> Result<String, std::io::Error> {
    fs::read_to_string(util::file::root_dir(&self.public_access_key)?)
  }

  pub fn read_private_refresh_key(&self) -> Result<String, std::io::Error> {
    fs::read_to_string(util::file::root_dir(&self.private_refresh_key)?)
  }

  pub fn read_public_refresh_key(&self) -> Result<String, std::io::Error> {
    fs::read_to_string(util::file::root_dir(&self.public_refresh_key)?)
  }
}

#[cfg(test)]
mod tests {
  use crate::CONFIG;

  #[test]
  fn test_read_private_access_key() {
    let key = CONFIG.secret.read_private_access_key().unwrap();
    assert!(!key.is_empty())
  }

  #[test]
  fn test_read_public_access_key() {
    let key = CONFIG.secret.read_public_access_key().unwrap();
    assert!(!key.is_empty())
  }

  #[test]
  fn test_read_private_refresh_key() {
    let key = CONFIG.secret.read_private_refresh_key().unwrap();
    assert!(!key.is_empty())
  }

  #[test]
  fn test_read_public_refresh_key() {
    let key = CONFIG.secret.read_public_refresh_key().unwrap();
    assert!(!key.is_empty())
  }
}
