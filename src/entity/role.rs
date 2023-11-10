use fake::Dummy;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};
use utoipa::ToSchema;

#[derive(
  Debug,
  PartialEq,
  Eq,
  strum::EnumString,
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
