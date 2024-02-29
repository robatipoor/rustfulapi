use std::time::Duration;

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use tracing::info;

use crate::configure::AppConfig;
use crate::error::AppResult;
use crate::util;

pub type DatabaseClient = DatabaseConnection;

pub trait DatabaseClientExt: Sized {
  fn build_from_config(config: &AppConfig) -> impl std::future::Future<Output = AppResult<Self>>;
}

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
  db.execute_unprepared(&format!("CREATE DATABASE {database_name}"))
    .await?;
  tracing::info!("Create new database: {database_name}.");
  Ok(())
}

pub async fn setup_new_database(config: &mut AppConfig) -> AppResult<DatabaseClient> {
  info!("Setup new database for the test.");
  let db = DatabaseClient::build_from_config(config).await?;
  config.db.database_name =
    util::random::generate_random_string_with_prefix("test_db").to_lowercase();
  create_database(&db, &config.db.database_name).await?;
  Ok(db)
}

pub async fn drop_database(db: &DatabaseConnection, database_name: &str) -> AppResult {
  let drop_query = format!("DROP DATABASE {database_name} WITH (FORCE);");
  db.execute_unprepared(&drop_query).await?;
  info!("Drop database: {database_name}.");
  Ok(())
}

pub async fn migrate_database(db: &DatabaseConnection) -> AppResult {
  info!("Start migrate database.");
  crate::migration::Migrator::up(db, None).await?;
  info!("Migrate database successfully done.");
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::constant::CONFIG;

  #[tokio::test]
  async fn test_ping_database() {
    DatabaseClient::build_from_config(&CONFIG)
      .await
      .unwrap()
      .ping()
      .await
      .expect("Database ping failed.")
  }
}
