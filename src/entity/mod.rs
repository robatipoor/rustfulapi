use sea_orm::{DatabaseTransaction, TransactionTrait};
use test_context::AsyncTestContext;
use tracing::info;

pub mod role;
pub mod user;

pub struct TransactionTestContext {
  pub tx: DatabaseTransaction,
}

#[async_trait::async_trait]
impl AsyncTestContext for TransactionTestContext {
  async fn setup() -> Self {
    info!("Setup database for the test.");
    let conn = crate::constant::DATABASE().await.unwrap();
    Self {
      tx: conn.begin().await.unwrap(),
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
