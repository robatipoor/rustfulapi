use sea_orm::{DatabaseTransaction, TransactionTrait};
use test_context::AsyncTestContext;
use tracing::info;

use crate::{
  client::database::{DatabaseClient, DatabaseClientExt},
  constant::CONFIG,
};

pub mod role;
pub mod user;

pub struct TransactionTestContext {
  pub tx: DatabaseTransaction,
}

#[async_trait::async_trait]
impl AsyncTestContext for TransactionTestContext {
  async fn setup() -> Self {
    info!("Setup database for the test.");
    let conn = DatabaseClient::build_from_config(&CONFIG).await.unwrap();
    Self {
      tx: conn.begin().await.unwrap(),
    }
  }

  async fn teardown(self) {
    self.tx.rollback().await.unwrap();
  }
}
