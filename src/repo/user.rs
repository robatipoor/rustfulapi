use chrono::Utc;
use sea_orm::{
  sea_query::Expr, ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection,
  DatabaseTransaction, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::{
  dto::{Direction, PageQueryParam},
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
  let mut user: entity::user::ActiveModel = user.into();
  user.is_active = Set(true);
  user.update(tx).await?;
  Ok(())
}

#[tracing::instrument]
pub async fn update_password(
  db: &DatabaseConnection,
  user_id: Uuid,
  password: String,
) -> AppResult<()> {
  entity::user::Entity::update_many()
    .col_expr(entity::user::Column::Password, Expr::value(password))
    .filter(entity::user::Column::Id.eq(user_id))
    .exec(db)
    .await?;
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
pub async fn find_page(
  conn: &DatabaseConnection,
  param: PageQueryParam,
) -> AppResult<Vec<entity::user::Model>> {
  let mut select = entity::user::Entity::find();
  match param.sort_direction {
    Some(Direction::DESC) => {
      // TODO fix me
      select = select.order_by_desc(entity::user::Column::CreateAt);
    }
    _ => {
      select = select.order_by_asc(entity::user::Column::CreateAt);
    }
  }
  let models = select
    .paginate(conn, param.page_size)
    .fetch_page(param.page_num)
    .await?;
  Ok(models)
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
