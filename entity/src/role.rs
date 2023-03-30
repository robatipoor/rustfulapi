use fake::Dummy;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};
use utoipa::ToSchema;

#[derive(
  Debug,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Deserialize,
  Serialize,
  ToSchema,
  Dummy,
  Clone,
  Copy,
  EnumIter,
  Display,
  Hash,
  sqlx::Type,
)]
pub enum RoleUser {
  Admin,
  User,
  System,
}

impl TryFrom<&str> for RoleUser {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "Admin" => Ok(Self::Admin),
      "User" => Ok(Self::User),
      "System" => Ok(Self::System),
      _ => Err("invalid role name"),
    }
  }
}
