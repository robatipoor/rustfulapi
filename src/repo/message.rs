use chrono::Utc;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, Condition, DatabaseTransaction, EntityTrait, QueryFilter,
  QueryOrder, Set,
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
pub async fn get_page(tx: &DatabaseTransaction) -> AppResult<Vec<entity::message::Model>> {
  let model = entity::message::Entity::find()
    .filter(
      entity::message::Column::Status
        .eq(MessageStatus::Pending)
        .or(entity::message::Column::Status.eq(MessageStatus::Failed)),
    )
    .cursor_by(entity::message::Column::CreateAt)
    .first(10)
    .all(tx)
    .await?;
  Ok(model)
}
