use chrono::{DateTime, Utc};
use fake::{Dummy, Fake};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, FromRow, Dummy)]
pub struct File {
  pub id: Uuid,
  pub user_id: Uuid,
  pub name: String,
  pub create_at: Option<DateTime<Utc>>,
  pub update_at: Option<DateTime<Utc>>,
}

impl File {
  pub fn new(name: impl Into<String>, user_id: Uuid) -> Self {
    Self {
      id: Uuid::new_v4(),
      user_id,
      name: name.into(),
      create_at: None,
      update_at: None,
    }
  }
}
