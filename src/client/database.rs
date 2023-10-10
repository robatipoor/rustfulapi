use std::time::Duration;

use async_trait::async_trait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::configure::AppConfig;
use crate::error::AppResult;

pub type DatabaseClient = DatabaseConnection;

#[async_trait]
pub trait DatabaseClientExt: Sized {
  async fn build_from_config(config: &AppConfig) -> AppResult<Self>;
  // async fn version(&self) -> Result<Option<String>, sea_orm::error::DbErr>;
}

#[async_trait]
impl DatabaseClientExt for DatabaseClient {
  async fn build_from_config(config: &AppConfig) -> AppResult<Self> {
    let mut opt = ConnectOptions::new(config.db.get_url());
    opt
      .max_connections(100)
      .min_connections(5)
      .connect_timeout(Duration::from_secs(8))
      .acquire_timeout(Duration::from_secs(8))
      .idle_timeout(Duration::from_secs(8))
      .max_lifetime(Duration::from_secs(8))
      .sqlx_logging(true)
      .sqlx_logging_level(log::LevelFilter::Info);
    let db = Database::connect(opt).await?;
    Ok(db)
  }
  // async fn version(&self) -> Result<Option<String>, sqlx::Error> {
  //   let version: Option<String> = sqlx::query!(r#"SELECT version()"#)
  //     .fetch_one(self)
  //     .await
  //     .map(|r| r.version)?;
  //   Ok(version)
  // }
}

// async fn get_pg_connection(pg_options: &PgConnectOptions) -> sqlx::Result<PgConnection> {
//   PgConnection::connect_with(pg_options).await
// }

// async fn create_database(db_name: &str, connection: &mut PgConnection) -> AppResult {
//   connection
//     .execute(&*format!("CREATE DATABASE {db_name}"))
//     .await?;
//   tracing::info!("create new database: {db_name}");
//   Ok(())
// }

// pub async fn setup_new_database(config: &mut DatabaseConfig) -> AppResult<PgConnection> {
//   info!("setup new postgres database for the test");
//   let mut pg_conn = get_pg_connection(&config.get_connection_options()).await?;
//   config.database_name = util::string::generate_random_string_with_prefix("test_db").to_lowercase();
//   create_database(&config.database_name, &mut pg_conn).await?;
//   Ok(pg_conn)
// }

// pub async fn drop_database(db_name: &str, connection: &mut PgConnection) -> AppResult {
//   let drop_query = format!("DROP DATABASE {db_name} WITH (FORCE);");
//   connection.execute(&*drop_query).await?;
//   info!("drop database: {db_name}");
//   Ok(())
// }

// pub async fn migrate_database(postgres: &PgClient) -> AppResult {
//   info!("migrate postgres database");
//   sqlx::migrate!("../migrations").run(postgres).await.unwrap();
//   info!("migrate database successfully done");
//   Ok(())
// }

#[cfg(test)]
mod tests {
  // use super::*;
  // use configure::CONFIG;
  // use sqlx::PgPool;

  // #[tokio::test]
  // async fn test_postgres_connection() {
  //   let client = PgPool::new(&CONFIG.db).await.unwrap();
  //   let version = client.version().await.unwrap().unwrap();
  //   assert!(!version.is_empty())
  // }
}
