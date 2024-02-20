use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
  Debug,
  PartialEq,
  Eq,
  strum::EnumString,
  PartialOrd,
  Ord,
  Deserialize,
  Serialize,
  utoipa::ToSchema,
  fake::Dummy,
  Clone,
  Copy,
  EnumIter,
  strum::Display,
  Hash,
  DeriveActiveEnum,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "ROLE_USER")]
pub enum RoleUser {
  #[sea_orm(string_value = "Admin")]
  Admin,
  #[sea_orm(string_value = "User")]
  User,
  #[sea_orm(string_value = "System")]
  System,
}
