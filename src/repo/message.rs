use chrono::Utc;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, EntityTrait,
  QueryFilter, Set, TransactionTrait,
};
use uuid::Uuid;

use crate::{
  entity::{
    self,
    message::{MessageKind, MessageStatus},
  },
  error::AppResult,
};

#[tracing::instrument(skip_all)]
pub async fn find_by_user_and_kind<C>(
  conn: &C,
  user_id: Uuid,
  kind: MessageKind,
) -> AppResult<Option<entity::message::Model>>
where
  C: ConnectionTrait,
{
  let model = entity::message::Entity::find()
    .filter(
      entity::message::Column::UserId
        .eq(user_id)
        .and(entity::message::Column::Kind.eq(kind)),
    )
    .one(conn)
    .await?;
  Ok(model)
}

#[tracing::instrument(skip_all)]
pub async fn save<C>(conn: &C, user_id: Uuid, content: String, kind: MessageKind) -> AppResult<Uuid>
where
  C: ConnectionTrait,
{
  let model = crate::entity::message::ActiveModel {
    id: Set(Uuid::new_v4()),
    content: Set(content),
    status: Set(crate::entity::message::MessageStatus::Pending),
    kind: Set(kind),
    user_id: Set(user_id),
    create_at: Set(Utc::now()),
    update_at: Set(Utc::now()),
  }
  .insert(conn)
  .await?;
  Ok(model.id)
}

#[tracing::instrument(skip_all)]
pub async fn get_list(
  conn: &DatabaseConnection,
  timeout: i64,
  limit: u64,
) -> AppResult<Vec<entity::message::Model>> {
  // Repeatable Read isolation applies when concurrent transactions attempting to update the same row will result in rollbacks.
  // Reference: https://www.postgresql.org/docs/current/transaction-iso.html
  let tx = conn
    .begin_with_config(Some(sea_orm::IsolationLevel::RepeatableRead), None)
    .await?;
  let results = get_list_and_update(&tx, timeout, limit).await?;
  tx.commit().await?;
  Ok(results)
}

#[tracing::instrument(skip_all)]
pub async fn get_list_and_update<C>(
  conn: &C,
  timeout: i64,
  limit: u64,
) -> AppResult<Vec<entity::message::Model>>
where
  C: ConnectionTrait,
{
  let models = entity::message::Entity::find()
    .filter(
      Condition::any()
        .add(entity::message::Column::Status.eq(MessageStatus::Pending))
        .add(entity::message::Column::Status.eq(MessageStatus::Failed))
        .add(
          // Resend the email if it times out.
          Condition::all()
            .add(entity::message::Column::Status.eq(MessageStatus::Sending))
            .add(
              entity::message::Column::UpdateAt
                .lte(Utc::now() - chrono::Duration::minutes(timeout)),
            ),
        ),
    )
    .cursor_by(entity::message::Column::CreateAt)
    .first(limit)
    .all(conn)
    .await?
    .into_iter()
    .map(|m| {
      let mut m: entity::message::ActiveModel = m.into();
      m.status = Set(MessageStatus::Sending);
      m
    })
    .collect::<Vec<_>>();
  let mut results = vec![];
  for model in models.into_iter() {
    results.push(model.update(conn).await?);
  }
  Ok(results)
}

#[tracing::instrument(skip_all)]
pub async fn update_status(
  conn: &DatabaseConnection,
  model: entity::message::Model,
  status: MessageStatus,
) -> AppResult {
  let mut model: entity::message::ActiveModel = model.into();
  model.status = Set(status);
  model.update(conn).await?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use chrono::Duration;
  use test_context::test_context;

  use crate::entity::TransactionTestContext;

  #[test_context(TransactionTestContext)]
  #[tokio::test]
  async fn test_get_list_messages(ctx: &mut TransactionTestContext) {
    let user_id = crate::entity::user::Entity::find()
      .one(&**ctx)
      .await
      .unwrap()
      .unwrap()
      .id;
    entity::message::ActiveModel {
      id: Set(Uuid::new_v4()),
      kind: Set(MessageKind::ActiveCode),
      status: Set(MessageStatus::Pending),
      content: Set("code1".to_string()),
      user_id: Set(user_id),
      create_at: Set(Utc::now() - Duration::seconds(100)),
      update_at: Set(Utc::now() - Duration::seconds(100)),
    }
    .insert(&**ctx)
    .await
    .unwrap();
    entity::message::ActiveModel {
      id: Set(Uuid::new_v4()),
      kind: Set(MessageKind::ActiveCode),
      status: Set(MessageStatus::Pending),
      content: Set("code2".to_string()),
      user_id: Set(user_id),
      create_at: Set(Utc::now() - Duration::seconds(10)),
      update_at: Set(Utc::now() - Duration::seconds(10)),
    }
    .insert(&**ctx)
    .await
    .unwrap();
    entity::message::ActiveModel {
      id: Set(Uuid::new_v4()),
      kind: Set(MessageKind::ActiveCode),
      status: Set(MessageStatus::Pending),
      content: Set("code3".to_string()),
      user_id: Set(user_id),
      create_at: Set(Utc::now()),
      update_at: Set(Utc::now()),
    }
    .insert(&**ctx)
    .await
    .unwrap();
    let list = get_list_and_update(&**ctx, 100, 2).await.unwrap();
    assert_eq!(list.len(), 2);
    let list = get_list_and_update(&**ctx, 100, 2).await.unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list.first().unwrap().content, "code3");
    let list = get_list_and_update(&**ctx, 100, 2).await.unwrap();
    assert_eq!(list.len(), 0);
  }
}
