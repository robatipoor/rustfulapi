use garde::Validate;
use sea_orm::DatabaseTransaction;
use sea_orm::TransactionTrait;
use tracing::info;
use uuid::Uuid;

use crate::constant::VERIFY_CODE_LEN;

use crate::dto::RegisterRequest;
use crate::entity::message::MessageKind;
use crate::repo;
// use crate::service::redis::*;
// use crate::service::session;
// use crate::service::token;
use crate::error::{AppError, AppResult};
use crate::server::state::AppState;
use crate::util;

pub async fn register(state: AppState, req: RegisterRequest) -> AppResult<Uuid> {
  info!("Register a new user request: {req:?}.");
  req.validate(&())?;
  let tx = state.db.begin().await?;
  check_unique_username_or_email(&tx, &req.username, &req.email).await?;
  let user_id = crate::repo::user::save(&tx, req.username, req.password, req.email).await?;
  let code = generate_invitation_code();
  crate::repo::message::save(&tx, user_id, code, MessageKind::ActiveCode).await?;
  state.messenger_notify.notify_one();
  tx.commit().await?;
  Ok(user_id)
}

pub fn generate_invitation_code() -> String {
  util::random::generate_random_string(VERIFY_CODE_LEN)
}

// pub async fn invitation(state: &AppState, req: InvitationRequest) -> AppResult<InvitationResponse> {
//   req.validate()?;
//   let user = fetch_user_by_email(&state.postgres, &req.email).await?;
//   if user.is_active {
//     return Err(AppError::Conflict("User is Already Active".to_string()));
//   }
//   util::password::verify(req.password.clone(), user.password.clone()).await?;
//   let (key, value) = crate::token::generate_invitation(user.id);
//   crate::redis::set(&state.redis, (&key, &value)).await?;
//   let invitation = Template::Invitation {
//     username: user.username.clone(),
//     code: value.code.clone(),
//   };
//   crate::email::send_email(&state.email, &invitation, "invitation email", &user.email).await?;
//   Ok(InvitationResponse::new(key.id, key.expire().as_secs()))
// }

// pub async fn active(state: &AppState, req: ActiveRequest) -> AppResult {
//   req.validate()?;
//   let key = InvitationKey { id: req.id };
//   let value = crate::redis::get(&state.redis, &key)
//     .await?
//     .ok_or_else(|| AppError::NotFound("Id is Not Found".to_string()))?;
//   if value.code != req.code {
//     return Err(invalid_input_error("code", "Code is Invalid"));
//   }
//   query::get_transaction(&state.postgres, move |mut tx| async move {
//     let mut user = fetch_user_by_id(&mut tx, &value.user_id).await?;
//     user.is_active = true;
//     query::user::update(&user).execute(&mut *tx).await?;
//     Ok(((), tx))
//   })
//   .await?;
//   crate::redis::del(&state.redis, &key).await?;
//   Ok(())
// }

// pub async fn validate(
//   state: &AppState,
//   user_id: &Uuid,
//   req: ValidateRequest,
// ) -> AppResult<UserClaims> {
//   info!("get validate token user_id: {user_id}");
//   req.validate()?;
//   let token_data = verify_access_token(&state.config.secret, &req.token)?;
//   session::check(&state.redis, &token_data.claims).await?;
//   Ok(token_data.claims)
// }

// pub async fn login(state: &AppState, req: LoginRequest) -> AppResult<LoginResponse> {
//   info!("user login req :{req:?}");
//   req.validate()?;
//   match req {
//     LoginRequest::Normal(req) => {
//       let user = fetch_active_user_by_email(&state.postgres, &req.email).await?;
//       util::password::verify(req.password.clone(), user.password.clone()).await?;
//       if user.is_tfa {
//         let (key, value) = generate_two_factor_login(user.id);
//         crate::redis::set(&state.redis, (&key, &value)).await?;
//         let login_template = Template::Login {
//           username: user.username,
//           code: value.code.clone(),
//         };
//         crate::email::send_email(&state.email, &login_template, "login email", &user.email).await?;
//         Ok(LoginResponse::Id { id: key.id })
//       } else {
//         let (key, value) = generate_session(user.id);
//         crate::redis::set(&state.redis, (&key, &value)).await?;
//         let resp = crate::token::generate_tokens(&state.config.secret, &user, &value.id)?;
//         // TODO log user login
//         Ok(LoginResponse::from(resp))
//       }
//     }
//     LoginRequest::TwoFactor(req) => {
//       let key = TwoFactorLoginKey { id: req.id };
//       let value = crate::redis::get(&state.redis, &key)
//         .await?
//         .ok_or_else(|| AppError::NotFound("Id Not Found".to_string()))?;
//       if value.code != req.code {
//         return Err(invalid_input_error("code", "Code is Invalid"));
//       }
//       let user = fetch_active_user(&state.postgres, value.user_id).await?;
//       let (key, value) = generate_session(user.id);
//       crate::redis::set(&state.redis, (&key, &value)).await?;
//       let resp = crate::token::generate_tokens(&state.config.secret, &user, &value.id)?;
//       // TODO log user login
//       Ok(LoginResponse::from(resp))
//     }
//   }
// }

// pub async fn refresh_token(state: &AppState, user_claims: &UserClaims) -> AppResult<TokenResponse> {
//   info!("start refresh token :{user_claims:?}");
//   let user_id = session::check(&state.redis, user_claims).await?;
//   let user = fetch_active_user(&state.postgres, user_id).await?;
//   info!("fetch active user :{}", user.id);
//   let (key, value) = token::generate_session(user.id);
//   info!("generate new session :{}", user.id);
//   crate::redis::set(&state.redis, (&key, &value)).await?;
//   let resp = token::generate_tokens(&state.config.secret, &user, &value.id)?;
//   info!("refresh token success :{user_claims:?}");
//   Ok(resp)
// }

// pub async fn logout(state: &AppState, user_id: Uuid) -> AppResult {
//   info!("user logout user id: {user_id}");
//   let key = SessionKey { user_id };
//   if !crate::redis::check_exist_key(&state.redis, &key).await? {
//     return Err(AppError::SessionNotExist("Session Not Found".to_string()));
//   }
//   crate::redis::del(&state.redis, &key).await?;
//   Ok(())
// }

// pub async fn forget_password(
//   state: &AppState,
//   req: ForgetPasswordParamQuery,
// ) -> AppResult<ForgetPasswordResponse> {
//   info!("forget password req: {req:?}");
//   req.validate()?;
//   let block_key = BlockEmailKey {
//     email: req.email.clone(),
//   };
//   if crate::redis::check_exist_key(&state.redis, &block_key).await? {
//     return Err(AppError::UserBlocked(
//       "This User was Temporary Blocked".to_string(),
//     ));
//   }
//   let user = fetch_active_user_by_email(&state.postgres, &req.email).await?;
//   let (key, value) = token::generate_forget_password(user.id);
//   crate::redis::set(&state.redis, (&key, &value)).await?;
//   let forget_pass = Template::ForgetPassword {
//     username: user.username,
//     code: value.code.clone(),
//   };
//   crate::email::send_email(&state.email, &forget_pass, "forget password", &user.email).await?;
//   let (block_key, value) = token::generate_block_email(req.email);
//   crate::redis::set(&state.redis, (&block_key, &value)).await?;
//   Ok(ForgetPasswordResponse { id: key.id })
// }

// pub async fn reset_password(state: &AppState, req: SetPasswordRequest) -> AppResult {
//   info!("reset password req: {req:?}");
//   req.validate()?;
//   let key = ForgetPasswordKey { id: req.id };
//   let value = crate::redis::get(&state.redis, &key)
//     .await?
//     .ok_or_else(|| AppError::NotFound("Id Not Found".to_string()))?;
//   if value.code != req.code {
//     crate::redis::del(&state.redis, &key).await?;
//     return Err(invalid_input_error("code", "Code is Invalid"));
//   }
//   let jh = tokio::task::spawn_blocking(move || hash::argon_hash(req.new_password));
//   let password = jh.await??;
//   query::get_transaction(&state.postgres, move |mut tx| async move {
//     let mut user = fetch_active_user_by_id(&mut tx, &value.user_id).await?;
//     user.password = password;
//     query::user::update(&user).execute(&mut *tx).await?;
//     Ok(((), tx))
//   })
//   .await?;

//   Ok(())
// }

// pub async fn get_profile(state: &AppState, user_id: &Uuid) -> AppResult<ProfileResponse> {
//   info!("get profile user id: {user_id}");
//   let user = fetch_active_user(&state.postgres, *user_id).await?;
//   Ok(ProfileResponse::from(&user))
// }

// pub async fn update_profile(
//   state: &AppState,
//   user_id: Uuid,
//   req: UpdateProfileRequest,
// ) -> AppResult {
//   info!("update profile user id: {user_id} req: {req:?}");
//   req.validate()?;
//   if let Some(username) = req.username.as_ref() {
//     if query::user::exist_by_username_or_email(username, username, None)
//       .fetch_one(&state.postgres)
//       .await?
//       .exist
//       .unwrap()
//     {
//       return Err(AppError::AlreadyExists(
//         "This Username Already Exists".to_string(),
//       ));
//     }
//   }
//   query::get_transaction(&state.postgres, move |mut tx| async move {
//     let mut user = fetch_active_user_by_id(&mut tx, &user_id).await?;
//     if let Some(is_tfa) = req.is_tfa {
//       user.is_tfa = is_tfa;
//     }
//     if let Some(username) = req.username {
//       user.username = username;
//     }
//     if let Some(password) = req.password {
//       user.password = password;
//     }
//     query::user::update(&user).execute(&mut *tx).await?;
//     Ok(((), tx))
//   })
//   .await?;
//   Ok(())
// }

// pub async fn fetch_user_by_id(
//   tx: &mut Transaction<'static, Postgres>,
//   user_id: &Uuid,
// ) -> AppResult<User> {
//   let user = query::user::find_by_id(user_id)
//     .fetch_optional(&mut **tx)
//     .await?
//     .ok_or_else(|| AppError::NotFound("No User Found with This ID".to_string()))?;
//   Ok(user)
// }

// pub async fn fetch_user_by_email(db: &PgClient, email: &str) -> AppResult<User> {
//   let user = query::user::find_by_email(email, None)
//     .fetch_optional(db)
//     .await?
//     .ok_or_else(|| AppError::NotFound("No User Found with This Email".to_string()))?;
//   Ok(user)
// }

// pub async fn fetch_active_user(db: &PgClient, user_id: Uuid) -> AppResult<User> {
//   query::get_transaction(db, move |mut tx| async move {
//     let user = fetch_active_user_by_id(&mut tx, &user_id).await?;
//     Ok(((user), tx))
//   })
//   .await
// }

// pub async fn fetch_active_user_by_id(
//   tx: &mut Transaction<'static, Postgres>,
//   user_id: &Uuid,
// ) -> AppResult<User> {
//   let user = fetch_user_by_id(tx, user_id).await?;
//   if !user.is_active {
//     return Err(AppError::UserNotActive(
//       "User is Not Currently Active".to_string(),
//     ));
//   }
//   Ok(user)
// }

// pub async fn fetch_active_user_by_email(db: &PgClient, email: &str) -> AppResult<User> {
//   let user = fetch_user_by_email(db, email).await?;
//   if !user.is_active {
//     return Err(AppError::UserNotActive(
//       "This User is Not Currently Active".to_string(),
//     ));
//   }
//   Ok(user)
// }

pub async fn check_unique_username_or_email(
  tx: &DatabaseTransaction,
  username: &str,
  email: &str,
) -> AppResult<()> {
  if repo::user::exist_by_username(tx, username, false).await? {
    Err(AppError::ResourceExistsError(crate::error::Resource {
      resource_type: crate::error::ResourceType::User,
      details: vec![("username".to_string(), username.to_string())],
    }))
  } else if repo::user::exist_by_email(tx, email, false).await? {
    Err(AppError::ResourceExistsError(crate::error::Resource {
      resource_type: crate::error::ResourceType::User,
      details: vec![("email".to_string(), email.to_string())],
    }))
  } else {
    Ok(())
  }
}
