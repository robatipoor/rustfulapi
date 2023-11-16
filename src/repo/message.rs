use argon2::password_hash::Output;
use chrono::Utc;
use sea_orm::{
  ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection,
  DatabaseTransaction, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use uuid::Uuid;

use crate::{
  client::database::DatabaseClientExt,
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

#[tracing::instrument(skip_all)]
pub async fn get_list(
  conn: &DatabaseConnection,
  limit: u64,
) -> AppResult<Vec<entity::message::Model>> {
  let tx = conn.begin().await?;
  let models = entity::message::Entity::find()
    .filter(
      Condition::any()
        .add(entity::message::Column::Status.eq(MessageStatus::Pending))
        .add(entity::message::Column::Status.eq(MessageStatus::Failed))
        .add(
          Condition::all()
            .add(entity::message::Column::Status.eq(MessageStatus::Sending))
            // Resend the email if it times out.
            .add(entity::message::Column::UpdateAt.lte(Utc::now() - chrono::Duration::minutes(5))),
        ),
    )
    .cursor_by(entity::message::Column::CreateAt)
    .first(limit)
    .all(&tx)
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
    results.push(model.update(&tx).await?);
  }
  tx.commit().await?;
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
    // let list = get_list(&**ctx, 2).await.unwrap();
    // assert_eq!(list.len(), 2);
    // let list = get_list(&**ctx, 1).await.unwrap();
    // assert_eq!(list.len(), 1);
    // assert_eq!(list.get(0).unwrap().content, "code1");
  }
}
