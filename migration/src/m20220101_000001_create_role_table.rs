use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::query::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let db = manager.get_connection();
    
    let tx = db.begin().await?;
    tx.execute_unprepared(
      r#"CREATE TABLE role (
            id uuid NOT NULL PRIMARY KEY,
            name varchar(255) NOT NULL
        )"#,
    )
    .await?;
    tx.execute_unprepared(
      r#"INSERT INTO role (id,name) VALUES ('a4ddad65-9277-426f-985e-2c6cde758e48','User'),
      ('e816b0ad-c02e-46e6-8a38-bda375c9cbe5','Admin'),('b1515b5c-5860-451b-8186-a800a59ee689','System')"#,
    )
    .await?;
    tx.commit().await?;
    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .get_connection()
      .execute_unprepared("DROP TABLE role")
      .await?;
    Ok(())
  }
}
