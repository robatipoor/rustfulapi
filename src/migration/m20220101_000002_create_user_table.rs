use sea_orm_migration::{prelude::*, sea_orm::TransactionTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let db = manager.get_connection();
    let tx = db.begin().await?;
    tx.execute_unprepared(
      r#"CREATE TABLE users (
            id UUID NOT NULL PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL UNIQUE,
            role ROLE_USER NOT NULL,
            is_active BOOLEAN NOT NULL,
            is_2fa BOOLEAN NOT NULL,
            create_at TIMESTAMPTZ DEFAULT current_timestamp,
            update_at TIMESTAMPTZ DEFAULT current_timestamp
        )"#,
    )
    .await?;
    tx.execute_unprepared(
      r#"INSERT INTO users (id, username, password, email, role, is_active, is_2fa, create_at, update_at) VALUES
   (
      gen_random_uuid(),
      'test-user',
      '$argon2id$v=19$m=4096,t=3,p=1$xj+gEfx2tF584ugWtZuZpw$t8MR3ns9T5n+0TsmUS3TGVQRmjRaoQVMyuBvvry1SbU',
      'test-user@email.com',
      'User',
      true,
      false,
      NOW(),
      NOW()
   )
   "#
    ).await?;
    tx.commit().await?;
    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .get_connection()
      .execute_unprepared("DROP TABLE IF EXISTS users")
      .await?;
    Ok(())
  }
}
