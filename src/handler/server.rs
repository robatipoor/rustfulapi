use crate::client::redis::RedisClientExt;
use crate::dto::{MessageResponse, ServiceStatusResponse};
use crate::error::AppResult;
use crate::server::state::AppState;
use axum::extract::State;
use axum::Json;
use tracing::error;

// Health check.
#[utoipa::path(
    get,
    path = "/api/v1/server/health_check",
    responses(
        (status = 200, description = "check service is up", body = [MessageResponse])
    )
)]
pub async fn health_check() -> AppResult<Json<MessageResponse>> {
  Ok(Json(MessageResponse::new("Ok")))
}

// Sever connection state.
#[utoipa::path(
    get,
    path = "/api/v1/server/state",
    responses(
        (status = 200, description = "state of connection services", body = [ServiceStatusResponse]),
        (status = 500, description = "internal server error", body = [AppResponseError])
    )
    // security(("jwt" = []))
)]
pub async fn server_state(State(state): State<AppState>) -> AppResult<Json<ServiceStatusResponse>> {
  let db = state.db.ping().await;
  if let Err(e) = db.as_ref() {
    error!("Database connection failed error: {e}.");
  }
  let email = state.email.test_connection().await;
  if let Err(e) = email.as_ref() {
    error!("Email service connection failed error: {e}.");
  }
  let redis = state.redis.ping().await;
  if let Err(e) = redis.as_ref() {
    error!("Redis connection failed error: {e}.");
  }
  let resp = ServiceStatusResponse {
    db: db.is_ok(),
    redis: redis.is_ok(),
    email: email.is_ok(),
  };
  Ok(Json(resp))
}

#[cfg(test)]
pub mod tests {

  use super::*;

  #[tokio::test]
  async fn test_health_check_handler() {
    assert_eq!(health_check().await.unwrap().0.message, "Ok");
  }
}
