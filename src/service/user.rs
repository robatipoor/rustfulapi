use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseTransaction;
use sea_orm::Set;
use sea_orm::TransactionTrait;
use tracing::info;
use uuid::Uuid;

use crate::constant::CODE_LEN;
use crate::dto::*;
use crate::entity;
use crate::entity::message::MessageKind;
use crate::error::invalid_input_error;
use crate::error::AppResult;
use crate::error::ToAppResult;
use crate::repo;
use crate::server::state::AppState;
use crate::service;
use crate::service::redis::ForgetPasswordKey;
use crate::service::redis::LoginKey;
use crate::service::redis::SessionKey;
use crate::service::token::verify_access_token;
use crate::util;
use crate::util::claim::UserClaims;

pub async fn register(state: AppState, req: RegisterRequest) -> AppResult<Uuid> {
  info!("Register a new user request: {req:?}.");
  let tx = state.db.begin().await?;
  check_unique_username_or_email(&tx, &req.username, &req.email).await?;
  let user_id = crate::repo::user::save(&tx, req.username, req.password, req.email).await?;
  let code = generate_active_code();
  service::message::store(
    &tx,
    &state.messenger_notify,
    user_id,
    code,
    MessageKind::ActiveCode,
  )
  .await?;
  tx.commit().await?;
  Ok(user_id)
}

pub fn generate_active_code() -> String {
  util::random::generate_random_string(CODE_LEN)
}

pub async fn active(state: &AppState, req: ActiveRequest) -> AppResult {
  let tx = state.db.begin().await?;
  let user = crate::repo::user::find_by_id(&tx, req.user_id)
    .await?
    .to_result()?;
  if user.is_active {
    return Ok(());
  }
  let message =
    crate::repo::message::find_by_user_and_kind(&tx, req.user_id, MessageKind::ActiveCode)
      .await?
      .to_result()?;
  if message.content != req.code {
    return Err(invalid_input_error("code", "Code is Invalid"));
  }
  crate::repo::user::active(&tx, user).await?;
  tx.commit().await?;
  Ok(())
}

pub async fn login(state: &AppState, req: LoginRequest) -> AppResult<Option<TokenResponse>> {
  info!("User login req :{req:?}");
  let user = crate::repo::user::find_by_email_and_status(&state.db, &req.email, true)
    .await?
    .to_result()?;
  util::password::verify(req.password.clone(), user.password.clone()).await?;
  if user.is_tfa {
    let code = service::token::generate_login_code(&state.redis, user.id).await?;
    crate::repo::message::save(&*state.db, user.id, code, MessageKind::LoginCode).await?;
    state.messenger_notify.notify_one();
    return Ok(None);
  }
  let session_id = service::session::set(&state.redis, user.id).await?;
  let resp = service::token::generate_tokens(&state.config.secret, user.id, user.role, session_id)?;
  Ok(Some(resp))
}

pub async fn two_factor_login(state: &AppState, req: TwoFactorLogin) -> AppResult<TokenResponse> {
  info!("User two factor login request: {req:?}");
  let key = LoginKey {
    user_id: req.user_id,
  };
  let code = service::redis::get(&state.redis, &key).await?;
  if code != Some(req.code) {
    return Err(invalid_input_error("code", "Code is Invalid"));
  }
  let user = crate::repo::user::find_by_id(&*state.db, req.user_id)
    .await?
    .to_result()?;
  let session_id = service::session::set(&state.redis, user.id).await?;
  service::token::generate_tokens(&state.config.secret, req.user_id, user.role, session_id)
}

pub async fn validate(
  state: &AppState,
  user_id: &Uuid,
  req: ValidateRequest,
) -> AppResult<UserClaims> {
  info!("Get validate token user_id: {user_id}");
  let token_data = verify_access_token(&state.config.secret, &req.token)?;
  service::session::check(&state.redis, &token_data.claims).await?;
  Ok(token_data.claims)
}

pub async fn refresh_token(state: &AppState, user_claims: &UserClaims) -> AppResult<TokenResponse> {
  info!("Refresh token: {user_claims:?}");
  let user_id = service::session::check(&state.redis, user_claims).await?;
  let user = crate::repo::user::find_by_id(&*state.db, user_id)
    .await?
    .to_result()?;
  let session_id = service::session::set(&state.redis, user.id).await?;
  info!("Set new session for user: {}", user.id);
  let resp = service::token::generate_tokens(&state.config.secret, user.id, user.role, session_id)?;
  info!("Refresh token success: {user_claims:?}");
  Ok(resp)
}

pub async fn logout(state: &AppState, user_id: Uuid) -> AppResult {
  info!("Logout user id: {user_id}");
  let key = SessionKey { user_id };
  service::redis::del(&state.redis, &key).await?;
  Ok(())
}

pub async fn forget_password(state: &AppState, req: ForgetPasswordParamQuery) -> AppResult {
  info!("Forget password req: {req:?}");
  let user = repo::user::find_by_email_and_status(&state.db, &req.email, true)
    .await?
    .to_result()?;
  if service::redis::check_exist_key(&state.redis, &ForgetPasswordKey { user_id: user.id }).await? {
    return Ok(());
  }
  let code = service::token::generate_forget_password_code(&state.redis, user.id).await?;
  service::message::store(
    &*state.db,
    &state.messenger_notify,
    user.id,
    code,
    MessageKind::ForgetPasswordCode,
  )
  .await?;
  Ok(())
}

pub async fn reset_password(state: &AppState, req: SetPasswordRequest) -> AppResult {
  info!("Reset password request: {req:?}");
  let code = service::redis::get(
    &state.redis,
    &ForgetPasswordKey {
      user_id: req.user_id,
    },
  )
  .await?;
  if code != Some(req.code) {
    return Err(invalid_input_error("code", "Code is Invalid"));
  }
  let password =
    tokio::task::spawn_blocking(move || crate::util::hash::argon_hash(req.new_password)).await??;
  let tx = state.db.begin().await?;
  let mut user: entity::user::ActiveModel = repo::user::find_by_id(&tx, req.user_id)
    .await?
    .to_result()?
    .into();
  user.password = Set(password);
  user.update(&tx).await?;
  tx.commit().await?;
  Ok(())
}

pub async fn get_profile(state: &AppState, user_id: Uuid) -> AppResult<ProfileResponse> {
  info!("Get user profile with id: {user_id}");
  let user = crate::repo::user::find_by_id(&*state.db, user_id)
    .await?
    .to_result()?;
  Ok(ProfileResponse::from(user))
}

pub async fn update_profile(
  state: &AppState,
  user_id: Uuid,
  req: UpdateProfileRequest,
) -> AppResult {
  info!("Update user profile with id: {user_id} req: {req:?}");
  let tx = state.db.begin().await?;
  if let Some(username) = req.username.as_ref() {
    repo::user::check_unique_by_username(&tx, username).await?;
  }
  let mut user: entity::user::ActiveModel = repo::user::find_by_id(&tx, user_id)
    .await?
    .to_result()?
    .into();
  if let Some(is_tfa) = req.is_tfa {
    user.is_tfa = Set(is_tfa);
  }
  if let Some(username) = req.username {
    user.username = Set(username);
  }
  if let Some(password) = req.password {
    user.password = Set(password);
  }
  user.update(&tx).await?;
  tx.commit().await?;
  Ok(())
}

pub async fn check_unique_username_or_email(
  tx: &DatabaseTransaction,
  username: &str,
  email: &str,
) -> AppResult {
  repo::user::check_unique_by_username(tx, username).await?;
  repo::user::check_unique_by_email(tx, email).await
}
