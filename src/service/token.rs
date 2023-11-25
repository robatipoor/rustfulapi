use std::time::Duration;

use jsonwebtoken::{DecodingKey, EncodingKey, TokenData};
use once_cell::sync::OnceCell;
use tracing::debug;
use uuid::Uuid;

use crate::client::redis::RedisClient;
use crate::configure::secret::SecretConfig;
use crate::constant::*;
use crate::dto::response::TokenResponse;
use crate::entity::role::RoleUser;
use crate::error::{AppError, AppResult};
use crate::util::{self, claim::UserClaims};

use crate::service::redis::*;
use crate::service::session;

use super::redis;

pub fn generate_block_email(email: String) -> (BlockEmailKey, BlockValue) {
  let block_id = Uuid::new_v4();
  let value = BlockValue { id: block_id };
  let key = BlockEmailKey { email };
  (key, value)
}

pub async fn generate_forget_password_code(
  redis: &RedisClient,
  user_id: Uuid,
) -> AppResult<String> {
  let code = util::random::generate_random_string(VERIFY_CODE_LEN);
  let key = ForgetPasswordKey { user_id };
  crate::service::redis::set(redis, (&key, &code)).await?;
  Ok(code)
}

pub async fn generate_login_code(redis: &RedisClient, user_id: Uuid) -> AppResult<String> {
  let login_code = util::random::generate_random_string(VERIFY_CODE_LEN);
  let key = LoginKey { user_id };
  redis::set(redis, (&key, &login_code)).await?;
  Ok(login_code)
}

// pub fn generate_invitation(user_id: Uuid) -> (InvitationKey, UserValue) {
//   let invitation_code = util::random::generate_random_string(VERIFY_CODE_LEN);
//   let invitation_id = Uuid::new_v4();
//   let value = UserValue::new(user_id, invitation_code);
//   let key = InvitationKey { id: invitation_id };
//   (key, value)
// }

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

// pub async fn generate_token_response(
//   redis: &RedisClient,
//   config: &SecretConfig,
//   expire_secs: u64,
//   user_id: Uuid,
//   role: RoleUser,
// ) -> AppResult<(UserClaims, TokenResponse)> {
//   let (key, value) = generate_session(user_id);
//   crate::service::redis::set(redis, (&key, &value)).await?;
//   let claims = UserClaims::new(Duration::from_secs(expire_secs), user_id, value.id, role);
//   let token = generate_tokens(config, user_id, role, value.id)?;
//   Ok((claims, token))
// }

pub fn generate_tokens(
  config: &SecretConfig,
  user_id: Uuid,
  role: RoleUser,
  session_id: Uuid,
) -> AppResult<TokenResponse> {
  let claims_access = UserClaims::new(EXPIRE_BEARER_TOKEN_SECS, user_id, session_id, role);
  let claims_refresh = UserClaims::new(EXPIRE_REFRESH_TOKEN_SECS, user_id, session_id, role);
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
  use fake::{Fake, Faker};
  use std::time::Duration;
  use uuid::Uuid;

  use crate::entity::role::RoleUser;
  use crate::service::token::*;

  #[test]
  fn test_generate_access_token() {
    let user_id: Uuid = Faker.fake();
    let session_id: Uuid = Faker.fake();
    let claims = UserClaims::new(Duration::from_secs(10), user_id, session_id, RoleUser::User);
    let token = encode_access_token(&CONFIG.secret, &claims).unwrap();
    assert!(!token.is_empty())
  }

  #[test]
  fn test_verify_access_token() {
    let user_id: Uuid = Faker.fake();
    let session_id: Uuid = Faker.fake();
    let claims = UserClaims::new(
      Duration::from_secs(100),
      user_id,
      session_id,
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
    let (key, session) = session::generate(user_id);
    let claims = UserClaims::new(Duration::from_secs(10), user_id, session.id, RoleUser::User);
    let token = encode_access_token(&CONFIG.secret, &claims).unwrap();
    set(&REDIS, (&key, &session)).await.unwrap();
    let claims = verify_token(&REDIS, &CONFIG.secret, &token, "/api/v1/resource")
      .await
      .unwrap()
      .claims;
    assert_eq!(claims.uid, user_id);
  }

  #[tokio::test]
  async fn test_verify_refresh_token() {
    let user_id: Uuid = Faker.fake();
    let (key, session) = session::generate(user_id);
    let claims = UserClaims::new(
      Duration::from_secs(10),
      user_id,
      session.id,
      RoleUser::System,
    );
    let token = encode_refresh_token(&CONFIG.secret, &claims).unwrap();
    set(&REDIS, (&key, &session)).await.unwrap();
    let claims = verify_token(&REDIS, &CONFIG.secret, &token, REFRESH_TOKEN_ROUTE)
      .await
      .unwrap()
      .claims;
    assert_eq!(claims.uid, user_id);
  }
}
