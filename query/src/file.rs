use model::Direction;
use sqlx::postgres::PgRow;
use sqlx::Postgres;
use sqlx::{postgres::PgArguments, query::Query};
use uuid::Uuid;

use entity::file::File;
use model::request::PageParamQuery;

use model::record::TotalRecord;

#[tracing::instrument]
pub fn save(item: &File) -> Query<Postgres, PgArguments> {
  assert!(item.create_at.is_none());
  assert!(item.update_at.is_none());
  sqlx::query!(
    r#"INSERT INTO file (id,user_id,name) VALUES ($1,$2,$3)"#,
    item.id,
    item.user_id,
    item.name,
  )
}

#[tracing::instrument]
pub fn delete_by_id(id: &Uuid) -> Query<Postgres, PgArguments> {
  sqlx::query!(r#"DELETE FROM file WHERE id = $1"#, id)
}

#[tracing::instrument]
pub fn find_by_id(
  id: &Uuid,
) -> sqlx::query::Map<'static, Postgres, impl FnMut(PgRow) -> Result<File, sqlx::Error>, PgArguments>
{
  sqlx::query_as!(File, r#"SELECT * FROM file WHERE id = $1"#, id)
}

#[tracing::instrument]
pub fn find_all_by_user(
  id: &Uuid,
) -> sqlx::query::Map<'static, Postgres, impl FnMut(PgRow) -> Result<File, sqlx::Error>, PgArguments>
{
  sqlx::query_as!(File, r#"SELECT * FROM file WHERE user_id = $1"#, id)
}

#[tracing::instrument]
pub async fn find_page(
  page: PageParamQuery,
) -> sqlx::query::Map<'static, Postgres, impl FnMut(PgRow) -> Result<File, sqlx::Error>, PgArguments>
{
  sqlx::query_as!(
    File,
    r#"SELECT * FROM file ORDER BY 
        (CASE WHEN $1 = 'create_at' AND $2 = 'ASC' THEN create_at END) ASC LIMIT $3 OFFSET $4"#,
    page.sort_by.unwrap_or("create_at".to_string()),
    page.sort_direction.unwrap_or(Direction::DESC).to_string(),
    page.page_size,
    page.page_num * page.page_size,
  )
}

#[tracing::instrument]
pub fn count_all() -> sqlx::query::Map<
  'static,
  Postgres,
  impl FnMut(PgRow) -> Result<TotalRecord, sqlx::Error>,
  PgArguments,
> {
  sqlx::query_as!(TotalRecord, r#"SELECT COUNT(1) AS total FROM file"#,)
}

#[cfg(test)]
mod tests {
  use entity::{file::File, User};
  use fake::{Fake, Faker};
  use test_context::{test_context, AsyncTestContext};

  use crate::TransactionTestContext;

  pub use super::*;

  pub struct TxRepoTestContext {
    pub tx_ctx: TransactionTestContext,
    pub user: User,
    pub file: File,
  }

  #[async_trait::async_trait]
  impl AsyncTestContext for TxRepoTestContext {
    async fn setup() -> Self {
      let mut tx_ctx = TransactionTestContext::setup().await;
      let mut user: User = Faker.fake();
      user.create_at = None;
      user.update_at = None;
      crate::user::save(&user)
        .execute(&mut tx_ctx.tx)
        .await
        .unwrap();
      let mut file: File = Faker.fake();
      file.create_at = None;
      file.update_at = None;
      file.user_id = user.id;
      crate::file::save(&file)
        .execute(&mut tx_ctx.tx)
        .await
        .unwrap();
      Self { tx_ctx, user, file }
    }

    async fn teardown(self) {
      self.tx_ctx.teardown().await;
    }
  }

  #[test_context(TxRepoTestContext)]
  #[tokio::test]
  async fn test_repo_save_and_find_file_by_id(ctx: &mut TxRepoTestContext) {
    let mut file: File = Faker.fake();
    file.create_at = None;
    file.update_at = None;
    file.user_id = ctx.user.id;
    let result = save(&file).execute(&mut ctx.tx_ctx.tx).await.unwrap();
    assert_eq!(result.rows_affected(), 1);
    let result = find_by_id(&file.id)
      .fetch_one(&mut ctx.tx_ctx.tx)
      .await
      .unwrap();
    assert_eq!(result.id, file.id);
    assert_eq!(result.name, file.name);
    assert_eq!(result.user_id, file.user_id);
    assert!(result.create_at.is_some());
    assert!(result.update_at.is_some());
  }
}
