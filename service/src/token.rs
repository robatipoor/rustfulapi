use std::time::Duration;

use jsonwebtoken::{DecodingKey, EncodingKey, TokenData};
use once_cell::sync::OnceCell;
use tracing::debug;
use uuid::Uuid;

use client::redis::RedisClient;
use configure::secret::SecretConfig;
use constant::*;
use entity::user::User;
use error::AppError;
use error::AppResult;
use model::response::TokenResponse;
use util::{self, claim::UserClaims};

use crate::redis::BlockEmailKey;
use crate::redis::BlockValue;
use crate::redis::ForgetPasswordKey;
use crate::redis::InvitationKey;
use crate::redis::SessionKey;
use crate::redis::SessionValue;
use crate::redis::TwoFactorLoginKey;
use crate::session;

use super::redis::UserValue;

pub fn generate_block_email(email: String) -> (BlockEmailKey, BlockValue) {
  let block_id = Uuid::new_v4();
  let value = BlockValue { id: block_id };
  let key = BlockEmailKey { email };
  (key, value)
}

pub fn generate_forget_password(user_id: Uuid) -> (ForgetPasswordKey, UserValue) {
  let reset_code = util::string::generate_random_string(VERIFY_CODE_LEN);
  let reset_id = Uuid::new_v4();
  let value = UserValue::new(user_id, reset_code);
  let key = ForgetPasswordKey { id: reset_id };
  (key, value)
}

pub fn generate_session(user_id: Uuid) -> (SessionKey, SessionValue) {
  let session_id = Uuid::new_v4();
  let value = SessionValue {
    user_id,
    id: session_id,
  };
  let key = SessionKey { user_id };
  (key, value)
}

pub fn generate_two_factor_login(user_id: Uuid) -> (TwoFactorLoginKey, UserValue) {
  let login_code = util::string::generate_random_string(VERIFY_CODE_LEN);
  let login_id = Uuid::new_v4();
  let value = UserValue::new(user_id, login_code);
  let key = TwoFactorLoginKey { id: login_id };
  (key, value)
}

pub fn generate_invitation(user_id: Uuid) -> (InvitationKey, UserValue) {
  let invitation_code = util::string::generate_random_string(VERIFY_CODE_LEN);
  let invitation_id = Uuid::new_v4();
  let value = UserValue::new(user_id, invitation_code);
  let key = InvitationKey { id: invitation_id };
  (key, value)
}

pub async fn verify_token(
  redis: &RedisClient,
  config: &SecretConfig,
  token: &str,
  req_path: &str,
) -> Result<TokenData<UserClaims>, AppError> {
  debug!("verify token: {token}");
  if REFRESH_TOKEN_ROUTE == req_path {
    let token_data = verify_refresh_token(config, token)?;
    return Ok(token_data);
  }
  let token_data = verify_access_token(config, token)?;
  session::check(redis, &token_data.claims).await?;
  Ok(token_data)
}

pub fn verify_access_token(config: &SecretConfig, token: &str) -> AppResult<TokenData<UserClaims>> {
  UserClaims::decode(token, get_access_token_decoding_key(config)?).map_err(|e| e.into())
}

pub fn verify_refresh_token(
  config: &SecretConfig,
  token: &str,
) -> AppResult<TokenData<UserClaims>> {
  UserClaims::decode(token, get_refresh_token_decoding_key(config)?).map_err(|e| e.into())
}

pub async fn generate_token_response(
  redis: &RedisClient,
  config: &SecretConfig,
  expire_secs: u64,
  user: &User,
) -> AppResult<(UserClaims, TokenResponse)> {
  let (key, value) = generate_session(user.id);
  crate::redis::set(redis, (&key, &value)).await?;
  let claims = UserClaims::new(
    Duration::from_secs(expire_secs),
    &user.id,
    &value.id,
    user.role_name,
  );
  let token = generate_tokens(config, user, &value.id)?;
  Ok((claims, token))
}

pub fn generate_tokens(
  config: &SecretConfig,
  user: &User,
  session_id: &Uuid,
) -> AppResult<TokenResponse> {
  let claims_access = UserClaims::new(
    EXPIRE_BEARER_TOKEN_SECS,
    &user.id,
    session_id,
    user.role_name,
  );
  let claims_refresh = UserClaims::new(
    EXPIRE_REFRESH_TOKEN_SECS,
    &user.id,
    session_id,
    user.role_name,
  );
  let access_token = encode_access_token(config, &claims_access)?;
  let refresh_token = encode_refresh_token(config, &claims_refresh)?;
  Ok(TokenResponse::new(
    access_token,
    refresh_token,
    EXPIRE_BEARER_TOKEN_SECS.as_secs(),
  ))
}

pub fn encode_access_token(config: &SecretConfig, claims: &UserClaims) -> AppResult<String> {
  claims
    .encode(get_access_token_encoding_key(config)?)
    .map_err(|e| e.into())
}

pub fn encode_refresh_token(config: &SecretConfig, claims: &UserClaims) -> AppResult<String> {
  claims
    .encode(get_refresh_token_encoding_key(config)?)
    .map_err(|e| e.into())
}

fn get_access_token_encoding_key(config: &SecretConfig) -> anyhow::Result<&'static EncodingKey> {
  static KEY: OnceCell<EncodingKey> = OnceCell::new();
  KEY.get_or_try_init(|| {
    let key = config.read_private_access_key()?;
    EncodingKey::from_rsa_pem(key.as_bytes()).map_err(|e| e.into())
  })
}

fn get_access_token_decoding_key(config: &SecretConfig) -> anyhow::Result<&'static DecodingKey> {
  static KEY: OnceCell<DecodingKey> = OnceCell::new();
  KEY.get_or_try_init(|| {
    let key = config.read_public_access_key()?;
    DecodingKey::from_rsa_pem(key.as_bytes()).map_err(|e| e.into())
  })
}

fn get_refresh_token_encoding_key(config: &SecretConfig) -> anyhow::Result<&'static EncodingKey> {
  static KEY: OnceCell<EncodingKey> = OnceCell::new();
  KEY.get_or_try_init(|| {
    let key = config.read_private_refresh_key()?;
    EncodingKey::from_rsa_pem(key.as_bytes()).map_err(|e| e.into())
  })
}

fn get_refresh_token_decoding_key(config: &SecretConfig) -> anyhow::Result<&'static DecodingKey> {
  static KEY: OnceCell<DecodingKey> = OnceCell::new();
  KEY.get_or_try_init(|| {
    let key = config.read_public_refresh_key()?;
    DecodingKey::from_rsa_pem(key.as_bytes()).map_err(|e| e.into())
  })
}

#[cfg(test)]
mod tests {
  use std::time::Duration;

  use client::redis::REDIS;
  use configure::CONFIG;
  use fake::{Fake, Faker};
  use uuid::Uuid;

  use crate::token::*;
  use entity::role::RoleUser;

  use crate::redis::*;

  #[test]
  fn test_generate_access_token() {
    let user_id: Uuid = Faker.fake();
    let session_id: Uuid = Faker.fake();
    let claims = UserClaims::new(
      Duration::from_secs(10),
      &user_id,
      &session_id,
      RoleUser::User,
    );
    let token = encode_access_token(&CONFIG.secret, &claims).unwrap();
    assert!(!token.is_empty())
  }

  #[test]
  fn test_verify_access_token() {
    let user_id: Uuid = Faker.fake();
    let session_id: Uuid = Faker.fake();
    let claims = UserClaims::new(
      Duration::from_secs(100),
      &user_id,
      &session_id,
      RoleUser::User,
    );
    let token = encode_access_token(&CONFIG.secret, &claims).unwrap();
    let result = verify_access_token(&CONFIG.secret, &token).unwrap();
    assert_eq!(result.claims.rol, claims.rol);
    assert_eq!(result.claims.exp, claims.exp);
    assert_eq!(result.claims.iat, claims.iat);
    assert_eq!(result.claims.sid, claims.sid);
    assert_eq!(result.claims.uid, claims.uid);
  }

  #[test]
  fn test_verify_invalid_access_token() {
    let token = r#"eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJpYXQiOjE2NjAwMjA1NjksImV4cCI6MTY2MDAyMDY2OSwidWlkIjoiT0FYazZaNVNKWjJyWXgiLCJzaWQiOiJjQllyd2lVSW83MTlhIiwicm9sIjoiVXNlciJ9.FLqRqLvzgUqlWJDaXgzroquL2cctF3kODx2vO3cwLq5D5mGa546nafwfxS2DVgs4pBFr_o4X6sP6uREhG5b7uT7SQ_D42aXw_wab-pVwuAsIYXXGIYt7CLB3krlA_kEAM3lceL5Th5qxdB7Mykp2NATh_4P-eBmQ0ads_gGGJLtfRdtfAvKtX0HPD3e-QWOOTnBhbJgxXgHhb6K17EbtlAn1XfmrpM81F_RZ6-oTE-DU-oQ4EbtSp1oMwAzIz4js9BEkBCEH6zV5DUJOmxLk66QuOMFeajgXoQ8qt6auknpXp8bi67LPE7v6G8WwmRcrCaQxHAuK_P-SZyVLi7j0mA"#;
    let result = verify_access_token(&CONFIG.secret, token);
    assert!(result.is_err());
  }

  #[tokio::test]
  async fn test_verify_token_access() {
    let user_id: Uuid = Faker.fake();
    let (key, session) = generate_session(user_id);
    let claims = UserClaims::new(
      Duration::from_secs(10),
      &user_id,
      &session.id,
      RoleUser::User,
    );
    let token = encode_access_token(&CONFIG.secret, &claims).unwrap();
    set(&REDIS, (&key, &session)).await.unwrap();
    let claims = verify_token(&REDIS, &CONFIG.secret, &token, "/api/v1/resource")
      .await
      .unwrap()
      .claims;
    assert_eq!(claims.uid, user_id.to_string());
  }

  #[tokio::test]
  async fn test_verify_refresh_token() {
    let user_id: Uuid = Faker.fake();
    let (key, session) = generate_session(user_id);
    let claims = UserClaims::new(
      Duration::from_secs(10),
      &user_id,
      &session.id,
      RoleUser::System,
    );
    let token = encode_refresh_token(&CONFIG.secret, &claims).unwrap();
    set(&REDIS, (&key, &session)).await.unwrap();
    let claims = verify_token(&REDIS, &CONFIG.secret, &token, REFRESH_TOKEN_ROUTE)
      .await
      .unwrap()
      .claims;
    assert_eq!(claims.uid, user_id.to_string());
  }
}
