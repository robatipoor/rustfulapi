use chrono::Utc;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, Condition, DatabaseTransaction, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

use crate::{
  entity::{
    self,
    message::{MessageKind, MessageStatus},
  },
  error::AppResult,
};

#[tracing::instrument]
pub async fn save(
  tx: &DatabaseTransaction,
  user_id: Uuid,
  content: String,
  kind: MessageKind,
) -> AppResult<Uuid> {
  let model = crate::entity::message::ActiveModel {
    id: Set(Uuid::new_v4()),
    content: Set(content),
    status: Set(crate::entity::message::MessageStatus::Pending),
    kind: Set(kind),
    user_id: Set(user_id),
    create_at: Set(Utc::now()),
    update_at: Set(Utc::now()),
  }
  .insert(tx)
  .await?;
  Ok(model.id)
}

#[tracing::instrument]
pub async fn get_list(
  tx: &DatabaseTransaction,
  limit: u64,
) -> AppResult<Vec<entity::message::Model>> {
  let model = entity::message::Entity::find()
    .filter(
      entity::message::Column::Status
        .eq(MessageStatus::Pending)
        .or(entity::message::Column::Status.eq(MessageStatus::Failed)),
    )
    .cursor_by(entity::message::Column::CreateAt)
    .first(limit)
    .all(tx)
    .await?;
  Ok(model)
}

#[cfg(test)]
mod tests {
  use chrono::{Duration, Utc};
  use sea_orm::{ActiveModelTrait, EntityTrait, Set};
  use test_context::test_context;
  use uuid::Uuid;

  use crate::entity::{
    self,
    message::{MessageKind, MessageStatus},
    TransactionTestContext,
  };

  use super::get_list;

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
      kind: Set(MessageKind::InvitationCode),
      status: Set(MessageStatus::Pending),
      content: Set("code".to_string()),
      user_id: Set(user_id),
      create_at: Set(Utc::now() - Duration::seconds(100)),
      update_at: Set(Utc::now() - Duration::seconds(100)),
    }
    .insert(&**ctx)
    .await
    .unwrap();
    entity::message::ActiveModel {
      id: Set(Uuid::new_v4()),
      kind: Set(MessageKind::InvitationCode),
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
      kind: Set(MessageKind::InvitationCode),
      status: Set(MessageStatus::Pending),
      content: Set("code3".to_string()),
      user_id: Set(user_id),
      create_at: Set(Utc::now()),
      update_at: Set(Utc::now()),
    }
    .insert(&**ctx)
    .await
    .unwrap();
    let list = get_list(&**ctx, 2).await.unwrap();
    assert_eq!(list.len(), 2);
  }
}
