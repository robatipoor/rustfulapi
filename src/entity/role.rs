use fake::Dummy;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "role")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: uuid::Uuid,
  pub name: RoleUser,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::Id",
    to = "super::user::Column::Id"
  )]
  User,
}

impl Related<crate::entity::user::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::User.def()
  }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

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
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "tea")]
pub enum RoleUser {
  #[sea_orm(string_value = "Admin")]
  Admin,
  #[sea_orm(string_value = "User")]
  User,
  #[sea_orm(string_value = "System")]
  System,
}
