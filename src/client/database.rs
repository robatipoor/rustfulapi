use std::time::Duration;

use async_trait::async_trait;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use tracing::info;

use crate::configure::AppConfig;
use crate::error::AppResult;
use crate::util;

pub type DatabaseClient = DatabaseConnection;

#[async_trait]
pub trait DatabaseClientExt: Sized {
  async fn build_from_config(config: &AppConfig) -> AppResult<Self>;
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
}

async fn create_database(db: &DatabaseConnection, database_name: &str) -> AppResult {
  db.execute_unprepared(&*format!("CREATE DATABASE {database_name}"))
    .await?;
  tracing::info!("Create new database: {database_name}.");
  Ok(())
}

pub async fn setup_new_database(config: &mut AppConfig) -> AppResult<DatabaseClient> {
  info!("Setup new postgres database for the test.");
  let db = DatabaseClient::build_from_config(config).await?;
  config.db.database_name =
    util::random::generate_random_string_with_prefix("test_db").to_lowercase();
  create_database(&db, &config.db.database_name).await?;
  Ok(db)
}

pub async fn drop_database(db: &DatabaseConnection, database_name: &str) -> AppResult {
  let drop_query = format!("DROP DATABASE {database_name} WITH (FORCE);");
  db.execute_unprepared(&*drop_query).await?;
  info!("Drop database: {database_name}.");
  Ok(())
}

// pub async fn migrate_database(postgres: &PgClient) -> AppResult {
//   info!("migrate postgres database");
//   sqlx::migrate!("../migrations").run(postgres).await.unwrap();
//   info!("migrate database successfully done");
//   Ok(())
// }

#[cfg(test)]
mod tests {
  use crate::constant::DATABASE;

  #[tokio::test]
  async fn test_ping_database() {
    DATABASE()
      .await
      .unwrap()
      .ping()
      .await
      .expect("Database ping failed.")
  }
}
