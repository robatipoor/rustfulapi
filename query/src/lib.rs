use futures::Future;
use sqlx::{PgConnection, PgPool, Postgres, Transaction};
use test_context::AsyncTestContext;
use tracing::info;

use client::postgres::{drop_database, migrate_database, setup_new_database, PgClient, PgPoolExt};
use configure::{db::DatabaseConfig, CONFIG};
use error::AppError;

pub mod user;

#[tracing::instrument(skip(f))]
pub async fn get_transaction<F, T, O>(pg_pool: &PgPool, f: F) -> Result<O, AppError>
where
  F: FnOnce(Transaction<'static, Postgres>) -> T + Send + Sync + 'static,
  O: Send + std::fmt::Debug + 'static,
  T: Future<Output = Result<(O, Transaction<'static, Postgres>), AppError>> + Send + 'static,
{
  let mut tx = pg_pool.begin().await.unwrap();
  // sqlx::query!(r#"SET TRANSACTION ISOLATION LEVEL REPEATABLE READ;"#)
  //     .execute(&mut tx)
  //     .await?;
  sqlx::query!(r#"SET statement_timeout=1000;"#)
    .execute(&mut tx)
    .await?;
  let (output, tx) = f(tx).await?;
  tx.commit().await?;
  info!("success commit output: {output:?}");
  Ok(output)
}

pub struct TransactionTestContext {
  pub tx: Transaction<'static, Postgres>,
}

#[async_trait::async_trait]
impl AsyncTestContext for TransactionTestContext {
  async fn setup() -> Self {
    info!("setup postgres db for the test");
    let postgres = PgClient::new(&CONFIG.db).await.unwrap();
    let tx = postgres.begin().await.unwrap();
    Self { tx }
  }

  async fn teardown(self) {
    self.tx.rollback().await.unwrap();
  }
}

pub struct PostgresTestContext {
  pub postgres: PgClient,
  pub config: DatabaseConfig,
  pub pg_conn: PgConnection,
}

#[async_trait::async_trait]
impl AsyncTestContext for PostgresTestContext {
  async fn setup() -> Self {
    let mut config = CONFIG.db.clone();
    let pg_conn = setup_new_database(&mut config).await.unwrap();
    let postgres = PgClient::new(&config).await.unwrap();
    migrate_database(&postgres).await.unwrap();
    Self {
      postgres,
      config,
      pg_conn,
    }
  }

  async fn teardown(mut self) {
    drop_database(&self.config.database_name, &mut self.pg_conn)
      .await
      .unwrap();
  }
}
