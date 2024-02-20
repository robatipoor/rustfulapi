use chrono::{DateTime, Utc};
use fake::Dummy;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::ToSchema;

use crate::error::ResourceType;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Dummy, DeriveEntityModel)]
#[sea_orm(table_name = "message")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  pub kind: MessageKind,
  pub status: MessageStatus,
  #[sea_orm(column_type = "Text")]
  pub content: String,
  pub user_id: Uuid,
  pub create_at: DateTime<Utc>,
  pub update_at: DateTime<Utc>,
}

impl super::AppEntity for Model {
  const RESOURCE: crate::error::ResourceType = ResourceType::Message;
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::user::Entity",
    from = "Column::UserId",
    to = "super::user::Column::Id"
  )]
  User,
}

impl Related<super::user::Entity> for Entity {
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
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "MESSAGE_KIND")]
pub enum MessageKind {
  #[sea_orm(string_value = "ActiveCode")]
  ActiveCode,
  #[sea_orm(string_value = "LoginCode")]
  LoginCode,
  #[sea_orm(string_value = "ForgetPasswordCode")]
  ForgetPasswordCode,
}

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
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "MESSAGE_STATUS")]
pub enum MessageStatus {
  #[sea_orm(string_value = "Pending")]
  Pending,
  #[sea_orm(string_value = "Sending")]
  Sending,
  #[sea_orm(string_value = "Success")]
  Success,
  #[sea_orm(string_value = "Failed")]
  Failed,
}
