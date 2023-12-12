use once_cell::sync::Lazy;
use regex::Regex;

pub static ALPHANUMERIC_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Za-z0-9]+$").unwrap());
pub static WORLD_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Za-z_]+$").unwrap());
pub static NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Za-z0-9_-]+$").unwrap());
