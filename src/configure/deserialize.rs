use serde::{Deserialize, Deserializer};
use std::time::Duration;

pub fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
  D: Deserializer<'de>,
{
  Ok(Duration::from_secs(u64::deserialize(deserializer)?))
}
