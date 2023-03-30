use client::redis::RedisClient;
use error::{AppError, AppResult};
use tracing::{debug, info};
use util::claim::UserClaims;
use uuid::Uuid;

use crate::redis::SessionKey;

pub async fn check(redis: &RedisClient, claims: &UserClaims) -> AppResult<Uuid> {
  let user_id = Uuid::parse_str(&claims.uid)?;
  let session_id = Uuid::parse_str(&claims.sid)?;
  let session_key = SessionKey { user_id };
  let session = crate::redis::get(redis, &session_key)
    .await?
    .ok_or_else(|| AppError::SessionNotExist("Session is Not Found".to_string()))?;
  if session.id != session_id {
    debug!("user: {user_id} unauthorized session_id: {session_id}");
    info!("session id invalid so deleting it: {session_key:?}");
    crate::redis::del(redis, &session_key).await?;
    return Err(AppError::InvalidSession("Session is Invalid".to_string()));
  }
  Ok(user_id)
}
