use axum::extract::State;
use axum::Json;
use garde::Validate;
use tracing::{info, warn};

use crate::error::AppResult;
use crate::server::state::AppState;
use crate::util::claim::UserClaims;
use crate::{dto::*, service};

/// Refresh token.
#[utoipa::path(
    post,
    path = "/api/v1/token/refresh",
    responses(
        (status = 200, description = "Success get new access token and refresh token", body = [TokenResponse]),
        (status = 400, description = "Invalid data input", body = [AppResponseError]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
)]
pub async fn refresh(
  State(state): State<AppState>,
  Json(req): Json<RefreshTokenRequest>,
) -> AppResult<Json<TokenResponse>> {
  info!("Refresh token with request: {req:?}.");
  match service::token::refresh(&state, req).await {
    Ok(resp) => {
      info!("Success refresh token user response: {resp:?}.");
      Ok(Json(resp))
    }
    Err(e) => {
      warn!("Unsuccessfully refresh token error: {e:?}.");
      Err(e)
    }
  }
}

/// Get token information.
#[utoipa::path(
    post,
    path = "/api/v1/token/info",
    request_body = TokenInfoRequest,
    responses(
        (status = 200, description = "Success get token information", body = [UserClaims]),
        (status = 400, description = "Invalid token", body = [AppResponseError]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn info(
  State(state): State<AppState>,
  user: UserClaims,
  Json(req): Json<TokenInfoRequest>,
) -> AppResult<Json<UserClaims>> {
  req.validate(&())?;
  info!("Get token information by user_id: {}.", user.uid);
  match service::token::info(&state, user, req).await {
    Ok(resp) => {
      info!("Success get token information response: {resp:?}.");
      Ok(Json(resp))
    }
    Err(e) => {
      warn!("Unsuccessfully get token information error: {e:?}.");
      Err(e)
    }
  }
}
