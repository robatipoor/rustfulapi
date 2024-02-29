use sea_orm::{DatabaseTransaction, TransactionTrait};
use test_context::AsyncTestContext;
use tracing::info;

use crate::{
  client::database::{DatabaseClient, DatabaseClientExt},
  constant::CONFIG,
  error::ResourceType,
};

pub mod message;
pub mod role;
pub mod user;

pub trait AppEntity {
  const RESOURCE: ResourceType;
}

pub struct TransactionTestContext {
  pub tx: DatabaseTransaction,
}

impl AsyncTestContext for TransactionTestContext {
  async fn setup() -> Self {
    info!("Setup database for the test.");
    let db = DatabaseClient::build_from_config(&CONFIG).await.unwrap();
    Self {
      tx: db.begin().await.unwrap(),
    }
  }

  async fn teardown(self) {
    self.tx.rollback().await.unwrap();
  }
}

impl std::ops::Deref for TransactionTestContext {
  type Target = DatabaseTransaction;

  fn deref(&self) -> &Self::Target {
    &self.tx
  }
}
