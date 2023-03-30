use std::collections::HashMap;

use client::postgres::PgClient;
use entity::*;
use error::AppResult;
use fake::{Fake, Faker};
use strum::IntoEnumIterator;
use util;
use uuid::Uuid;

pub struct TestUser {
  pub id: Uuid,
  pub email: String,
  pub password: String,
}

impl TestUser {
  pub async fn create_users(pg_client: &PgClient) -> AppResult<HashMap<RoleUser, TestUser>> {
    let mut users = HashMap::<RoleUser, TestUser>::new();
    for role in RoleUser::iter() {
      let mut user: User = Faker.fake();
      let password = user.password.clone();
      user.is_active = true;
      user.role_name = role;
      user.is_tfa = false;
      user.password = util::password::hash(password.clone()).await?;
      user.create_at = None;
      user.update_at = None;
      query::user::save(&user).execute(pg_client).await?;
      let test_user = TestUser {
        id: user.id,
        email: user.email,
        password,
      };
      users.insert(user.role_name, test_user);
    }
    Ok(users)
  }
}
