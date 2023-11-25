use crate::client::redis::RedisClient;
use crate::error::{AppError, AppResult};
use crate::util::claim::UserClaims;
use tracing::{debug, info};
use uuid::Uuid;

use crate::service::redis::SessionKey;

use super::redis::SessionValue;

pub async fn check(redis: &RedisClient, claims: &UserClaims) -> AppResult<Uuid> {
  let user_id = claims.uid;
  let session_id = claims.sid;
  let session_key = SessionKey { user_id };
  let session = crate::service::redis::get(redis, &session_key)
    .await?
    .ok_or_else(|| {
      AppError::NotFoundError(crate::error::Resource {
        details: vec![],
        resource_type: crate::error::ResourceType::Session,
      })
    })?;
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

pub async fn set(redis: &RedisClient, user_id: Uuid) -> AppResult<Uuid> {
  let (key, value) = generate(user_id);
  crate::service::redis::set(redis, (&key, &value)).await?;
  Ok(value.id)
}

pub fn generate(user_id: Uuid) -> (SessionKey, SessionValue) {
  let session_id = Uuid::new_v4();
  let value = SessionValue {
    user_id,
    id: session_id,
  };
  let key = SessionKey { user_id };
  (key, value)
}
