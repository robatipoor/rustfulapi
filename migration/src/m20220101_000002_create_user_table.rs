use sea_orm_migration::prelude::*;

use super::m20220101_000002_seed_role_table::Role;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let db = manager.get_connection();
    let tx = db.begin().await?;
    tx.execute_unprepared(
      r#"CREATE TABLE users (
            id uuid NOT NULL PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL UNIQUE,
            role_name RoleUser NOT NULL,
            is_active BOOLEAN NOT NULL,
            is_tfa BOOLEAN NOT NULL,
            create_at timestamptz DEFAULT current_timestamp,
            update_at timestamptz DEFAULT current_timestamp
        )"#,
    )
    .await?;
    tx.commit().await?;
    tx.execute_unprepared(
      r#"INSERT INTO users (id,username,password,email,role_name,is_active,is_tfa) VALUES (gen_random_uuid(),
      'test-user','$argon2id$v=19$m=4096,t=3,p=1$xj+gEfx2tF584ugWtZuZpw$t8MR3ns9T5n+0TsmUS3TGVQRmjRaoQVMyuBvvry1SbU',
      'test-user@email.com','User',true,false)"#
    );
    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.drop_table("DROP TABLE users").await
  }
}
