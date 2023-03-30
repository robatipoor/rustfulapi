use async_trait::async_trait;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use tracing::info;

use configure::db::DatabaseConfig;
use error::AppResult;

pub type PgClient = PgPool;

#[async_trait]
pub trait PgPoolExt: Sized {
  async fn new(config: &DatabaseConfig) -> Result<Self, sqlx::Error>;
  async fn version(&self) -> Result<Option<String>, sqlx::Error>;
}

#[async_trait]
impl PgPoolExt for PgPool {
  async fn new(config: &DatabaseConfig) -> Result<PgClient, sqlx::Error> {
    PgPoolOptions::new()
      .max_connections(config.max_connections)
      .connect(&config.get_url())
      .await
  }
  async fn version(&self) -> Result<Option<String>, sqlx::Error> {
    let version: Option<String> = sqlx::query!(r#"SELECT version()"#)
      .fetch_one(self)
      .await
      .map(|r| r.version)?;
    Ok(version)
  }
}

async fn get_pg_connection(pg_options: &PgConnectOptions) -> sqlx::Result<PgConnection> {
  PgConnection::connect_with(pg_options).await
}

async fn create_database(db_name: &str, connection: &mut PgConnection) -> AppResult {
  connection
    .execute(&*format!("CREATE DATABASE {db_name}"))
    .await?;
  tracing::info!("create new database: {db_name}");
  Ok(())
}

pub async fn setup_new_database(config: &mut DatabaseConfig) -> AppResult<PgConnection> {
  info!("setup new postgres database for the test");
  let mut pg_conn = get_pg_connection(&config.get_connection_options()).await?;
  config.database_name = util::string::generate_random_string_with_prefix("test_db").to_lowercase();
  create_database(&config.database_name, &mut pg_conn).await?;
  Ok(pg_conn)
}

pub async fn drop_database(db_name: &str, connection: &mut PgConnection) -> AppResult {
  let drop_query = format!("DROP DATABASE {db_name} WITH (FORCE);");
  connection.execute(&*drop_query).await?;
  info!("drop database: {db_name}");
  Ok(())
}

pub async fn migrate_database(postgres: &PgClient) -> AppResult {
  info!("migrate postgres database");
  sqlx::migrate!("../migrations").run(postgres).await.unwrap();
  info!("migrate database successfully done");
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use configure::CONFIG;
  use sqlx::PgPool;

  #[tokio::test]
  async fn test_postgres_connection() {
    let client = PgPool::new(&CONFIG.db).await.unwrap();
    let version = client.version().await.unwrap().unwrap();
    assert!(!version.is_empty())
  }
}
