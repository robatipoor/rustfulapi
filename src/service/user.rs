use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseTransaction;
use sea_orm::Set;
use sea_orm::TransactionTrait;
use tracing::info;
use uuid::Uuid;

use crate::constant::CHECK_EMAIL_MESSAGE;
use crate::constant::CODE_LEN;
use crate::constant::EXPIRE_FORGET_PASS_CODE_SECS;
use crate::constant::EXPIRE_TWO_FACTOR_CODE_SECS;
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
use crate::util;

pub async fn register(state: AppState, req: RegisterRequest) -> AppResult<Uuid> {
  info!("Register a new user request: {req:?}.");
  let tx = state.db.begin().await?;
  check_unique_username_or_email(&tx, &req.username, &req.email).await?;
  let user_id = crate::repo::user::save(&tx, req.username, req.password, req.email).await?;
  let code = generate_active_code();
  repo::message::save(&tx, user_id, code, MessageKind::ActiveCode).await?;
  tx.commit().await?;
  state.messenger_notify.notify_one();
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

pub async fn login(state: &AppState, req: LoginRequest) -> AppResult<LoginResponse> {
  info!("User login request :{req:?}.");
  let user = crate::repo::user::find_by_email_and_status(&state.db, &req.email, true)
    .await?
    .to_result()?;
  util::password::verify(req.password.clone(), user.password.clone()).await?;
  if user.is_2fa {
    let key = LoginKey { user_id: user.id };
    let ttl = service::redis::get_tll(&state.redis, &key).await?;
    if ttl > 0 {
      return Ok(LoginResponse::Code {
        expire_in: ttl as u64,
        message: CHECK_EMAIL_MESSAGE.to_string(),
      });
    }
    let login_code = util::random::generate_random_string(CODE_LEN);
    crate::repo::message::save(
      &*state.db,
      user.id,
      login_code.clone(),
      MessageKind::LoginCode,
    )
    .await?;
    crate::service::redis::set(&state.redis, (&key, &login_code)).await?;
    state.messenger_notify.notify_one();
    return Ok(LoginResponse::Code {
      expire_in: EXPIRE_TWO_FACTOR_CODE_SECS.as_secs(),
      message: CHECK_EMAIL_MESSAGE.to_string(),
    });
  }
  let session_id = service::session::set(&state.redis, user.id).await?;
  let resp = service::token::generate_tokens(user.id, user.role, session_id)?;
  Ok(LoginResponse::Token(resp))
}

pub async fn login2fa(state: &AppState, req: Login2faRequest) -> AppResult<TokenResponse> {
  info!("User two factor login request: {req:?}");
  let key = LoginKey {
    user_id: req.user_id,
  };
  let code = service::redis::get(&state.redis, &key).await?;
  if code != Some(req.code) {
    return Err(invalid_input_error("code", "Code is invalid."));
  }
  let user = crate::repo::user::find_by_id(&*state.db, req.user_id)
    .await?
    .to_result()?;
  let session_id = service::session::set(&state.redis, user.id).await?;
  service::token::generate_tokens(req.user_id, user.role, session_id)
}

pub async fn logout(state: &AppState, user_id: Uuid) -> AppResult {
  info!("Logout user id: {user_id}");
  let key = SessionKey { user_id };
  service::redis::del(&state.redis, &key).await?;
  Ok(())
}

pub async fn forget_password(
  state: &AppState,
  req: ForgetPasswordQueryParam,
) -> AppResult<ForgetPasswordResponse> {
  info!("Forget password request: {req:?}");
  let user = repo::user::find_by_email_and_status(&state.db, &req.email, true)
    .await?
    .to_result()?;
  let key = ForgetPasswordKey { user_id: user.id };
  let ttl = service::redis::get_tll(&state.redis, &key).await?;
  if ttl > 0 {
    return Ok(ForgetPasswordResponse {
      expire_in: ttl as u64,
      message: CHECK_EMAIL_MESSAGE.to_string(),
    });
  }
  let code = util::random::generate_random_string(CODE_LEN);
  repo::message::save(
    &*state.db,
    user.id,
    code.clone(),
    MessageKind::ForgetPasswordCode,
  )
  .await?;
  crate::service::redis::set(&state.redis, (&key, &code)).await?;
  state.messenger_notify.notify_one();
  Ok(ForgetPasswordResponse {
    expire_in: EXPIRE_FORGET_PASS_CODE_SECS.as_secs(),
    message: CHECK_EMAIL_MESSAGE.to_string(),
  })
}

pub async fn reset_password(state: &AppState, req: SetPasswordRequest) -> AppResult {
  info!("Reset password user: {}", req.user_id);
  let code = service::redis::get(
    &state.redis,
    &ForgetPasswordKey {
      user_id: req.user_id,
    },
  )
  .await?;
  if code != Some(req.code) {
    return Err(invalid_input_error("code", "Code is invalid"));
  }
  let password =
    tokio::task::spawn_blocking(move || crate::util::hash::argon_hash(req.new_password)).await??;
  repo::user::update_password(&state.db, req.user_id, password).await?;
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
  info!("Update user profile with id: {user_id} request: {req:?}");
  let tx = state.db.begin().await?;
  if let Some(username) = req.username.as_ref() {
    repo::user::check_unique_by_username(&tx, username).await?;
  }
  let mut user: entity::user::ActiveModel = repo::user::find_by_id(&tx, user_id)
    .await?
    .to_result()?
    .into();
  if let Some(is_2fa) = req.is_2fa {
    user.is_2fa = Set(is_2fa);
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
