use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::query::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let db = manager.get_connection();

    let tx = db.begin().await?;
    tx.execute_unprepared(r#"CREATE TYPE ROLE_USER AS ENUM ('Admin', 'User', 'System')"#)
      .await?;
    tx.commit().await?;
    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .get_connection()
      .execute_unprepared("DROP TYPE IF EXISTS ROLE_USER")
      .await?;
    Ok(())
  }
}
