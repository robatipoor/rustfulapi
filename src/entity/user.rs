use chrono::{DateTime, Utc};
use fake::faker::internet::en::{FreeEmail, Password, Username};
use fake::Dummy;
use sea_orm::entity::prelude::*;
use uuid::Uuid;

use super::role::RoleUser;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Dummy, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uuid,
  #[dummy(faker = "Username()")]
  #[sea_orm(column_type = "Text", unique, indexed)]
  pub username: String,
  #[dummy(faker = "Password(8..100)")]
  #[sea_orm(column_type = "Text")]
  pub password: String,
  #[dummy(faker = "FreeEmail()")]
  #[sea_orm(column_type = "Text", unique, indexed)]
  pub email: String,
  pub role: RoleUser,
  pub is_active: bool,
  pub is_tfa: bool,
  pub create_at: Option<DateTime<Utc>>,
  pub update_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}
