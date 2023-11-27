use sea_orm::ConnectionTrait;
use tokio::sync::Notify;
use uuid::Uuid;

use crate::{entity::message::MessageKind, error::AppResult};

pub async fn store<C>(
  conn: &C,
  messenger_notify: &Notify,
  user_id: Uuid,
  content: String,
  kind: MessageKind,
) -> AppResult
where
  C: ConnectionTrait,
{
  crate::repo::message::save(conn, user_id, content, kind).await?;
  messenger_notify.notify_one();
  Ok(())
}
