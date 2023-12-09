use chrono::Utc;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection,
  DatabaseTransaction, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::{
  entity,
  error::{AppResult, ToAppResult},
  util,
};

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
    is_2fa: Set(false),
    create_at: Set(Utc::now()),
    update_at: Set(Utc::now()),
  }
  .insert(tx)
  .await?;
  Ok(user.id)
}

#[tracing::instrument]
pub async fn active(tx: &DatabaseTransaction, user: entity::user::Model) -> AppResult<()> {
  let user: entity::user::ActiveModel = user.into();
  user.update(tx).await?;
  Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn find_by_id<C>(conn: &C, id: Uuid) -> AppResult<Option<entity::user::Model>>
where
  C: ConnectionTrait,
{
  let model = entity::user::Entity::find_by_id(id).one(conn).await?;
  Ok(model)
}

#[tracing::instrument(skip_all)]
pub async fn find_by_email_and_status(
  conn: &DatabaseConnection,
  email: &str,
  is_active: bool,
) -> AppResult<Option<entity::user::Model>> {
  let model = entity::user::Entity::find()
    .filter(
      entity::user::Column::Email
        .eq(email)
        .and(entity::user::Column::IsActive.eq(is_active)),
    )
    .one(conn)
    .await?;
  Ok(model)
}

#[tracing::instrument]
pub async fn exist_by_email_and_status(
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
pub async fn check_unique_by_email(tx: &DatabaseTransaction, email: &str) -> AppResult {
  crate::entity::user::Entity::find()
    .filter(crate::entity::user::Column::Email.eq(email))
    .one(tx)
    .await?
    .check_absent_details(vec![("email".to_string(), email.to_string())])
}

#[tracing::instrument]
pub async fn check_unique_by_username(tx: &DatabaseTransaction, username: &str) -> AppResult {
  crate::entity::user::Entity::find()
    .filter(crate::entity::user::Column::Username.eq(username))
    .one(tx)
    .await?
    .check_absent_details(vec![("username".to_string(), username.to_string())])
}

#[tracing::instrument]
pub async fn exist_by_username_and_status(
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
