use chrono::Utc;
use sqlx::postgres::PgRow;
use sqlx::Postgres;
use sqlx::{postgres::PgArguments, query::Query};
use uuid::Uuid;

use entity::{role::RoleUser, user::User};
use model::request::{Direction, PageParamQuery};

use model::record::*;

#[tracing::instrument]
pub fn save(item: &User) -> Query<Postgres, PgArguments> {
  assert!(item.create_at.is_none());
  assert!(item.update_at.is_none());
  sqlx::query!(
    r#"INSERT INTO users (id,username,password,email,role_name,is_active,is_tfa) VALUES ($1,$2,$3,$4,$5,$6,$7)"#,
    item.id,
    item.username,
    item.password,
    item.email,
    item.role_name as _,
    item.is_active,
    item.is_tfa,
  )
}

#[tracing::instrument]
pub fn find_by_id(
  id: &Uuid,
) -> sqlx::query::Map<'static, Postgres, impl FnMut(PgRow) -> Result<User, sqlx::Error>, PgArguments>
{
  sqlx::query_as!(
    User,
    r#"SELECT id,username,password,email,role_name as "role_name: _",
        is_active,is_tfa,create_at,update_at FROM users WHERE id = $1"#,
    id
  )
}

#[tracing::instrument]
pub fn find_by_email(
  email: &str,
  is_active: Option<bool>,
) -> sqlx::query::Map<'static, Postgres, impl FnMut(PgRow) -> Result<User, sqlx::Error>, PgArguments>
{
  sqlx::query_as!(
    User,
    r#"SELECT id,username,password,email,role_name as "role_name: _",
    is_active,is_tfa,create_at,update_at FROM users WHERE (is_active = $1 OR $1 IS NULL) AND email = $2"#,
    is_active,
    email,
  )
}

#[tracing::instrument]
pub fn count_all() -> sqlx::query::Map<
  'static,
  Postgres,
  impl FnMut(PgRow) -> Result<TotalRecord, sqlx::Error>,
  PgArguments,
> {
  sqlx::query_as!(TotalRecord, r#"SELECT COUNT(1) AS total FROM users"#,)
}

#[tracing::instrument]
pub fn find_page(
  page: PageParamQuery,
) -> sqlx::query::Map<'static, Postgres, impl FnMut(PgRow) -> Result<User, sqlx::Error>, PgArguments>
{
  sqlx::query_as!(
    User,
    r#"SELECT id,username,password,email,role_name as "role_name:RoleUser",is_active,
    is_tfa,create_at,update_at FROM users ORDER BY 
            (CASE WHEN $1 = 'create_at' AND $2 = 'ASC' THEN create_at END) ASC,
            (CASE WHEN $1 = 'create_at' AND $2 = 'DESC' THEN create_at END) DESC,
            (CASE WHEN $1 = 'username' AND $2 = 'ASC' THEN username END) ASC,
            (CASE WHEN $1 = 'username' AND $2 = 'DESC' THEN username END) DESC
            LIMIT $3 OFFSET $4"#,
    page.sort_by.unwrap_or("create_at".to_string()),
    page.sort_direction.unwrap_or(Direction::DESC).to_string(),
    page.page_size,
    page.page_num * page.page_size,
  )
}

#[tracing::instrument]
pub fn update(item: &User) -> Query<Postgres, PgArguments> {
  sqlx::query!(
    r#"UPDATE users SET username=$1,password=$2,email=$3,role_name=$4,is_active=$5,is_tfa=$6 WHERE id = $7"#,
    item.username,
    item.password,
    item.email,
    item.role_name as _,
    item.is_active,
    item.is_tfa,
    item.id,
  )
}

#[tracing::instrument]
pub fn exist_by_username_or_email(
  username: &str,
  email: &str,
  is_active: Option<bool>,
) -> sqlx::query::Map<
  'static,
  Postgres,
  impl FnMut(PgRow) -> Result<ExistRecord, sqlx::Error>,
  PgArguments,
> {
  sqlx::query_as!(
    ExistRecord,
    r#"SELECT EXISTS(SELECT 1 FROM users WHERE (is_active = $1 OR $1 IS NULL) 
            AND (username = $2 OR email = $3)) AS exist"#,
    is_active,
    username,
    email,
  )
}

#[tracing::instrument]
pub fn delete_all_inative_user() -> Query<'static, Postgres, PgArguments> {
  sqlx::query!(
    r#"DELETE FROM users WHERE is_active = $1 AND create_at <= $2"#,
    false,
    Utc::now() - chrono::Duration::days(1)
  )
}

#[cfg(test)]
mod tests {
  use fake::{Fake, Faker};
  use rand::seq::SliceRandom;
  use test_context::{test_context, AsyncTestContext};

  use crate::TransactionTestContext;

  pub use super::*;

  pub struct UserTxRepoTestContext {
    pub tx_ctx: TransactionTestContext,
    pub users: [User; 5],
  }

  #[async_trait::async_trait]
  impl AsyncTestContext for UserTxRepoTestContext {
    async fn setup() -> Self {
      let mut tx_ctx = TransactionTestContext::setup().await;
      let mut users: [User; 5] = Faker.fake();
      for user in users.iter_mut() {
        user.create_at = None;
        user.update_at = None;
        save(user).execute(&mut tx_ctx.tx).await.unwrap();
      }
      Self { tx_ctx, users }
    }

    async fn teardown(self) {
      self.tx_ctx.teardown().await;
    }
  }

  #[test_context(UserTxRepoTestContext)]
  #[tokio::test]
  pub async fn test_repo_save_and_find_user_by_id(ctx: &mut UserTxRepoTestContext) {
    let mut user: User = Faker.fake();
    user.create_at = None;
    user.update_at = None;
    let result = save(&user).execute(&mut ctx.tx_ctx.tx).await.unwrap();
    assert_eq!(result.rows_affected(), 1);
    let result = find_by_id(&user.id)
      .fetch_one(&mut ctx.tx_ctx.tx)
      .await
      .unwrap();
    assert_eq!(result.id, user.id);
    assert_eq!(result.username, user.username);
    assert_eq!(result.password, user.password);
    assert_eq!(result.email, user.email);
    assert_eq!(result.is_active, user.is_active);
    assert_eq!(result.is_tfa, user.is_tfa);
  }

  #[test_context(UserTxRepoTestContext)]
  #[tokio::test]
  async fn test_repo_exist_user_by_username_or_email_repo(ctx: &mut UserTxRepoTestContext) {
    let user = ctx.users.choose(&mut rand::thread_rng()).unwrap();
    let result = exist_by_username_or_email(&user.username, &user.email, Some(user.is_active))
      .fetch_one(&mut ctx.tx_ctx.tx)
      .await
      .unwrap();
    assert!(result.exist.unwrap());
    let result = exist_by_username_or_email(&user.username, "fake_user1234@mail.com", None)
      .fetch_one(&mut ctx.tx_ctx.tx)
      .await
      .unwrap();
    assert!(result.exist.unwrap());
    let result = exist_by_username_or_email(&user.username, &user.email, None)
      .fetch_one(&mut ctx.tx_ctx.tx)
      .await
      .unwrap();
    assert!(result.exist.unwrap());
    let result =
      exist_by_username_or_email("fake_username", "fake_user@mail.com", Some(user.is_active))
        .fetch_one(&mut ctx.tx_ctx.tx)
        .await
        .unwrap();
    assert!(!result.exist.unwrap());
  }

  #[test_context(UserTxRepoTestContext)]
  #[tokio::test]
  async fn test_repo_find_user_by_email(ctx: &mut UserTxRepoTestContext) {
    let user = ctx.users.choose(&mut rand::thread_rng()).unwrap();
    let result = find_by_email(&user.email, Some(user.is_active))
      .fetch_one(&mut ctx.tx_ctx.tx)
      .await
      .unwrap();
    assert_eq!(result.username, user.username);
  }

  #[test_context(UserTxRepoTestContext)]
  #[tokio::test]
  async fn test_repo_find_page_user(ctx: &mut UserTxRepoTestContext) {
    let req = PageParamQuery {
      page_num: 0,
      page_size: 2,
      sort_by: Some("username".to_string()),
      sort_direction: Some(Direction::ASC),
    };
    let result1 = find_page(req).fetch_all(&mut ctx.tx_ctx.tx).await.unwrap();
    let req = PageParamQuery {
      page_num: 2,
      page_size: 2,
      sort_by: Some("username".to_string()),
      sort_direction: Some(Direction::ASC),
    };
    let result2 = find_page(req).fetch_all(&mut ctx.tx_ctx.tx).await.unwrap();
    assert_eq!(result1.len(), 2);
    assert_eq!(result2.len(), 2);
  }

  #[test_context(UserTxRepoTestContext)]
  #[tokio::test]
  async fn test_repo_update_user(ctx: &mut UserTxRepoTestContext) {
    let mut user = ctx.users.choose(&mut rand::thread_rng()).unwrap().clone();
    let old_pass = user.password.clone();
    let new_pass: String = Faker.fake();
    user.password = new_pass.clone();
    let expected_pass = new_pass.clone();
    update(&user).execute(&mut ctx.tx_ctx.tx).await.unwrap();
    let user_id = user.id;
    let result = find_by_id(&user_id)
      .fetch_one(&mut ctx.tx_ctx.tx)
      .await
      .unwrap();
    assert_eq!(result.password, expected_pass);
    assert_ne!(result.password, old_pass)
  }
}
