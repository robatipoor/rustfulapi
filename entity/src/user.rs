use chrono::{DateTime, Utc};
use fake::faker::internet::en::{FreeEmail, Password, Username};
use fake::Dummy;
use sqlx::FromRow;
use uuid::Uuid;

use super::role::RoleUser;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Dummy, FromRow)]
pub struct User {
  pub id: Uuid,
  #[dummy(faker = "Username()")]
  pub username: String,
  #[dummy(faker = "Password(8..100)")]
  pub password: String,
  #[dummy(faker = "FreeEmail()")]
  pub email: String,
  pub role_name: RoleUser,
  pub is_active: bool,
  pub is_tfa: bool,
  pub create_at: Option<DateTime<Utc>>,
  pub update_at: Option<DateTime<Utc>>,
}

impl User {
  pub fn new(
    username: impl Into<String>,
    password: impl Into<String>,
    email: impl Into<String>,
    role_name: RoleUser,
  ) -> Self {
    Self {
      id: Uuid::new_v4(),
      username: username.into(),
      password: password.into(),
      email: email.into(),
      role_name,
      is_active: false,
      is_tfa: false,
      create_at: None,
      update_at: None,
    }
  }
}
