use crate::client::redis::RedisClient;
use crate::error::{AppError, AppResult};
use crate::util::claim::UserClaims;
use tracing::{debug, info};
use uuid::Uuid;

use crate::service::redis::SessionKey;

pub async fn check(redis: &RedisClient, claims: &UserClaims) -> AppResult<Uuid> {
  let user_id = claims.uid;
  let session_id = claims.sid;
  let session_key = SessionKey { user_id };
  let session = crate::service::redis::get(redis, &session_key)
    .await?
    .ok_or_else(|| AppError::NotFoundError(crate::error::ResourceType::Session, vec![]))?;
  if session.id != session_id {
    debug!("user: {user_id} unauthorized session_id: {session_id}");
    info!("session id invalid so deleting it: {session_key:?}");
    crate::service::redis::del(redis, &session_key).await?;
    return Err(AppError::InvalidSessionError(
      "Session is Invalid".to_string(),
    ));
  }
  Ok(user_id)
}
