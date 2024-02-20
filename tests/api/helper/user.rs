use std::collections::HashMap;

use chrono::Utc;
use fake::faker::internet::en::FreeEmail;
use fake::{Fake, Faker};
use rustfulapi::{entity, util};
use rustfulapi::{entity::role::RoleUser, error::AppResult};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Iterable, Set};
use uuid::Uuid;

pub struct TestUser {
  pub id: Uuid,
  pub username: String,
  pub email: String,
  pub password: String,
}

impl TestUser {
  pub async fn create_users(db: &DatabaseConnection) -> AppResult<HashMap<RoleUser, TestUser>> {
    let mut users = HashMap::<RoleUser, TestUser>::new();
    for role in RoleUser::iter() {
      let password: String = Faker.fake();
      let user = entity::user::ActiveModel {
        password: Set(util::password::hash(password.clone()).await?),
        id: Set(Uuid::new_v4()),
        username: Set(Faker.fake::<String>()),
        email: Set(FreeEmail().fake::<String>()),
        role: Set(role),
        is_active: Set(true),
        is_2fa: Set(false),
        create_at: Set(Utc::now()),
        update_at: Set(Utc::now()),
      };
      let user = user.insert(db).await?;
      let test_user = TestUser {
        id: user.id,
        email: user.email,
        username: user.username,
        password,
      };
      users.insert(user.role, test_user);
    }
    Ok(users)
  }
}
