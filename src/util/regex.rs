use regex::Regex;
use std::sync::LazyLock;

pub static ALPHANUMERIC_REGEX: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^[A-Za-z0-9]+$").unwrap());
pub static WORLD_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[A-Za-z_]+$").unwrap());
pub static NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[A-Za-z0-9_-]+$").unwrap());
