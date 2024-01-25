use crate::constant::*;
use crate::dto::response::TokenResponse;
use crate::dto::{RefreshTokenRequest, TokenInfoRequest};
use crate::entity::role::RoleUser;
use crate::error::{AppError, AppResult, ToAppResult};
use crate::server::state::AppState;
use crate::service;
use crate::util::claim::UserClaims;
use tracing::info;
use uuid::Uuid;

pub async fn info(
  state: &AppState,
  user: UserClaims,
  req: TokenInfoRequest,
) -> AppResult<UserClaims> {
  info!("Get token info by user_id: {}", user.uid);
  if user.rol != RoleUser::System {
    return Err(AppError::PermissionDeniedError(
      "This user does not have permission to use this resource.".to_string(),
    ));
  }
  let token_data = UserClaims::decode(&req.token, &ACCESS_TOKEN_DECODE_KEY)?;
  service::session::check(&state.redis, &token_data.claims).await?;
  Ok(token_data.claims)
}

pub async fn refresh(state: &AppState, req: RefreshTokenRequest) -> AppResult<TokenResponse> {
  let user_claims = UserClaims::decode(&req.token, &REFRESH_TOKEN_DECODE_KEY)?.claims;
  info!("Refresh token: {user_claims:?}");
  let user_id = service::session::check(&state.redis, &user_claims).await?;
  let user = crate::repo::user::find_by_id(&*state.db, user_id)
    .await?
    .to_result()?;
  let session_id = service::session::set(&state.redis, user.id).await?;
  info!("Set new session for user: {}", user.id);
  let resp = generate_tokens(user.id, user.role, session_id)?;
  info!("Refresh token success: {user_claims:?}");
  Ok(resp)
}

pub fn generate_tokens(
  user_id: Uuid,
  role: RoleUser,
  session_id: Uuid,
) -> AppResult<TokenResponse> {
  let access_token = UserClaims::new(EXPIRE_BEARER_TOKEN_SECS, user_id, session_id, role)
    .encode(&ACCESS_TOKEN_ENCODE_KEY)?;
  let refresh_token = UserClaims::new(EXPIRE_REFRESH_TOKEN_SECS, user_id, session_id, role)
    .encode(&REFRESH_TOKEN_ENCODE_KEY)?;
  Ok(TokenResponse::new(
    access_token,
    refresh_token,
    EXPIRE_BEARER_TOKEN_SECS.as_secs(),
  ))
}
