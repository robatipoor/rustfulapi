use sea_orm_migration::{prelude::*, sea_orm::TransactionTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let db = manager.get_connection();
    let tx = db.begin().await?;
    tx.execute_unprepared(
      r#"CREATE TYPE MESSAGE_KIND AS ENUM ('ActiveCode', 'LoginCode', 'ForgetPasswordCode')"#,
    )
    .await?;
    tx.execute_unprepared(
      r#"CREATE TYPE MESSAGE_STATUS AS ENUM ('Pending', 'Sending', 'Success', 'Failed')"#,
    )
    .await?;
    tx.execute_unprepared(
      r#"CREATE TABLE message (
            id UUID NOT NULL PRIMARY KEY,
            kind MESSAGE_KIND NOT NULL,
            content TEXT NOT NULL,
            status MESSAGE_STATUS NOT NULL,
            user_id UUID NOT NULL,
            create_at TIMESTAMPTZ DEFAULT current_timestamp,
            update_at TIMESTAMPTZ DEFAULT current_timestamp,
            CONSTRAINT fk_message_user FOREIGN KEY(user_id) REFERENCES users(id)
        )"#,
    )
    .await?;

    tx.commit().await?;
    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let conn = manager.get_connection();
    let tx = conn.begin().await?;
    tx.execute_unprepared("ALTER TABLE message DROP CONSTRAINT fk_message_user")
      .await?;
    tx.execute_unprepared("DROP TABLE IF EXISTS message")
      .await?;
    tx.execute_unprepared("DROP TYPE IF EXISTS MESSAGE_KIND")
      .await?;
    tx.execute_unprepared("DROP TYPE IF EXISTS MESSAGE_STATUS")
      .await?;
    tx.commit().await?;
    Ok(())
  }
}
