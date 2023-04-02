use actix_web::{web, HttpResponse};

use client::postgres::PgPoolExt;
use client::redis::RedisClientExt;
use error::{AppError, AppResult};
use model::response::MessageResponse;
use model::*;
use state::AppState;
use tracing::error;

// check server health check
#[utoipa::path(
    get,
    path = "/api/v1/server/health_check",
    responses(
        (status = 200, description = "check service is up", body = [MessageResponse])
    )
)]
pub async fn health_check() -> Result<HttpResponse, AppError> {
  Ok(HttpResponse::Ok().json(MessageResponse::new("ok")))
}

// check server connection state
#[utoipa::path(
    get,
    path = "/api/v1/server/state",
    responses(
        (status = 200, description = "state of connection services", body = [ServiceStatusResponse]),
        (status = 500, description = "internal server error", body = [AppResponseError])
    )
    // security(("jwt" = []))
)]
pub async fn server_state(state: web::Data<AppState>) -> AppResult<HttpResponse> {
  let postgres = state.postgres.version().await;
  if let Err(e) = postgres.as_ref() {
    error!("postgres connection failed error: {e}");
  }
  let email = state.email.test_connection().await;
  if let Err(e) = email.as_ref() {
    error!("email service connection failed error: {e}");
  }
  let redis = state.redis.ping().await;
  if let Err(e) = redis.as_ref() {
    error!("redis connection failed error: {e}");
  }
  let resp = ServiceStatusResponse {
    postgres: postgres.is_ok(),
    redis: redis.is_ok(),
    email: email.is_ok(),
  };
  Ok(HttpResponse::Ok().json(resp))
}

#[cfg(test)]
pub mod tests {
  use super::*;

  #[tokio::test]
  async fn health_check_handler_test() {
    assert_eq!(
      health_check().await.unwrap().status(),
      HttpResponse::Ok().finish().status()
    );
  }
}
