use chrono::Utc;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseTransaction, EntityTrait,
  QueryFilter, Set,
};
use uuid::Uuid;

use crate::{entity, error::AppResult, util};

#[tracing::instrument]
pub async fn save(
  tx: &DatabaseTransaction,
  username: String,
  password: String,
  email: String,
) -> AppResult<Uuid> {
  let user = crate::entity::user::ActiveModel {
    id: Set(Uuid::new_v4()),
    username: Set(username),
    password: Set(util::password::hash(password).await?),
    email: Set(email),
    role: Set(crate::entity::role::RoleUser::User),
    is_active: Set(false),
    is_tfa: Set(false),
    create_at: Set(Utc::now()),
    update_at: Set(Utc::now()),
  }
  .insert(tx)
  .await?;
  Ok(user.id)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_id<C>(conn: &C, id: Uuid) -> AppResult<Option<entity::user::Model>>
where
  C: ConnectionTrait,
{
  let model = entity::user::Entity::find_by_id(id).one(conn).await?;
  Ok(model)
}

// #[tracing::instrument]
// pub fn find_by_id(
//   id: &Uuid,
// ) -> sqlx::query::Map<'static, Postgres, impl FnMut(PgRow) -> Result<User, sqlx::Error>, PgArguments>
// {
//   sqlx::query_as!(
//     User,
//     r#"SELECT id,username,password,email,role_name as "role_name: _",
//         is_active,is_tfa,create_at,update_at FROM users WHERE id = $1"#,
//     id
//   )
// }

// #[tracing::instrument]
// pub fn find_by_email(
//   email: &str,
//   is_active: Option<bool>,
// ) -> sqlx::query::Map<'static, Postgres, impl FnMut(PgRow) -> Result<User, sqlx::Error>, PgArguments>
// {
//   sqlx::query_as!(
//     User,
//     r#"SELECT id,username,password,email,role_name as "role_name: _",
//     is_active,is_tfa,create_at,update_at FROM users WHERE (is_active = $1 OR $1 IS NULL) AND email = $2"#,
//     is_active,
//     email,
//   )
// }

// #[tracing::instrument]
// pub fn count_all() -> sqlx::query::Map<
//   'static,
//   Postgres,
//   impl FnMut(PgRow) -> Result<TotalRecord, sqlx::Error>,
//   PgArguments,
// > {
//   sqlx::query_as!(TotalRecord, r#"SELECT COUNT(1) AS total FROM users"#,)
// }

// #[tracing::instrument]
// pub fn find_page(
//   page: PageParamQuery,
// ) -> sqlx::query::Map<'static, Postgres, impl FnMut(PgRow) -> Result<User, sqlx::Error>, PgArguments>
// {
//   sqlx::query_as!(
//     User,
//     r#"SELECT id,username,password,email,role_name as "role_name:RoleUser",is_active,
//     is_tfa,create_at,update_at FROM users ORDER BY
//             (CASE WHEN $1 = 'create_at' AND $2 = 'ASC' THEN create_at END) ASC,
//             (CASE WHEN $1 = 'create_at' AND $2 = 'DESC' THEN create_at END) DESC,
//             (CASE WHEN $1 = 'username' AND $2 = 'ASC' THEN username END) ASC,
//             (CASE WHEN $1 = 'username' AND $2 = 'DESC' THEN username END) DESC
//             LIMIT $3 OFFSET $4"#,
//     page.sort_by.unwrap_or("create_at".to_string()),
//     page.sort_direction.unwrap_or(Direction::DESC).to_string(),
//     page.page_size,
//     page.page_num * page.page_size,
//   )
// }

// #[tracing::instrument]
// pub fn update(item: &User) -> Query<Postgres, PgArguments> {
//   sqlx::query!(
//     r#"UPDATE users SET username=$1,password=$2,email=$3,role_name=$4,is_active=$5,is_tfa=$6 WHERE id = $7"#,
//     item.username,
//     item.password,
//     item.email,
//     item.role_name as _,
//     item.is_active,
//     item.is_tfa,
//     item.id,
//   )
// }

#[tracing::instrument]
pub async fn exist_by_email(
  tx: &DatabaseTransaction,
  email: &str,
  is_active: bool,
) -> AppResult<bool> {
  Ok(
    crate::entity::user::Entity::find()
      .filter(
        Condition::all()
          .add(crate::entity::user::Column::IsActive.eq(is_active))
          .add(crate::entity::user::Column::Email.contains(email)),
      )
      .one(tx)
      .await?
      .is_some(),
  )
}
#[tracing::instrument]
pub async fn exist_by_username(
  tx: &DatabaseTransaction,
  username: &str,
  is_active: bool,
) -> AppResult<bool> {
  Ok(
    crate::entity::user::Entity::find()
      .filter(
        Condition::all()
          .add(crate::entity::user::Column::IsActive.eq(is_active))
          .add(crate::entity::user::Column::Username.contains(username)),
      )
      .one(tx)
      .await?
      .is_some(),
  )
}
