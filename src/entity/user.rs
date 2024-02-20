use chrono::{DateTime, Utc};
use fake::faker::internet::en::{FreeEmail, Password, Username};
use fake::Dummy;
use sea_orm::entity::prelude::*;

use crate::error::ResourceType;

use super::role::RoleUser;
use super::AppEntity;

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
  pub is_2fa: bool,
  pub create_at: DateTime<Utc>,
  pub update_at: DateTime<Utc>,
}

impl AppEntity for Model {
  const RESOURCE: ResourceType = ResourceType::User;
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::message::Entity")]
  Message,
}

impl Related<super::message::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Message.def()
  }
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
pub mod tests {

  use fake::{Fake, Faker};
  use sea_orm::Set;
  use test_context::test_context;

  use crate::entity::TransactionTestContext;

  use super::*;

  #[test_context(TransactionTestContext)]
  #[tokio::test]
  async fn test_insert_and_find_user_entity(ctx: &mut TransactionTestContext) {
    let id = Uuid::new_v4();
    let username: String = Faker.fake();
    let password: String = Faker.fake();
    let email: String = Faker.fake();
    ActiveModel {
      id: Set(id),
      username: Set(username.clone()),
      password: Set(password.clone()),
      email: Set(email.clone()),
      role: Set(fake::Faker.fake()),
      is_active: Set(fake::Faker.fake()),
      is_2fa: Set(fake::Faker.fake()),
      create_at: Set(fake::Faker.fake()),
      update_at: Set(fake::Faker.fake()),
    }
    .insert(&**ctx)
    .await
    .unwrap();
    let user = super::Entity::find_by_id(id)
      .one(&**ctx)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(user.password, password);
    assert_eq!(user.username, username);
    assert_eq!(user.email, email);
  }
}
