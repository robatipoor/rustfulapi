use std::time::Duration;

use actix_web::http::header::HeaderMap;
use actix_web::{HttpMessage, HttpRequest};
use anyhow::anyhow;
use chrono::Utc;
use fake::Dummy;
use jsonwebtoken::Header;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, TokenData, Validation};
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use entity::role::RoleUser;
use error::{AppError, AppResult};

pub static DECODE_HEADER: Lazy<Validation> = Lazy::new(|| Validation::new(Algorithm::RS256));
pub static ENCODE_HEADER: Lazy<Header> = Lazy::new(|| Header::new(Algorithm::RS256));

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Dummy, ToSchema)]
pub struct UserClaims {
  // issued at
  pub iat: i64,
  // expiration
  pub exp: i64,
  // user id
  pub uid: String,
  // session id
  pub sid: String,
  // role user
  pub rol: RoleUser,
}

impl UserClaims {
  pub fn new(duration: Duration, user_id: &Uuid, session_id: &Uuid, role: RoleUser) -> Self {
    let now = Utc::now().timestamp();
    Self {
      iat: now,
      exp: now + (duration.as_secs() as i64),
      uid: user_id.to_string(),
      sid: session_id.to_string(),
      rol: role,
    }
  }

  pub fn decode(
    token: &str,
    key: &DecodingKey,
  ) -> Result<TokenData<UserClaims>, jsonwebtoken::errors::Error> {
    jsonwebtoken::decode::<UserClaims>(token, key, Lazy::force(&DECODE_HEADER))
  }

  pub fn encode(&self, key: &EncodingKey) -> Result<String, jsonwebtoken::errors::Error> {
    jsonwebtoken::encode(Lazy::force(&ENCODE_HEADER), self, key)
  }
}

pub fn parse_bearer_token_from_header(headers: &HeaderMap) -> Result<String, anyhow::Error> {
  let auth_header = headers
    .get(constant::AUTHORIZATION)
    .ok_or_else(|| anyhow!("header auth not found"))?;
  let auth_header = auth_header.to_str()?;
  if let Some(token) = auth_header.strip_prefix(constant::BEARER) {
    Ok(token.trim().to_string())
  } else {
    Err(anyhow!("auth header not start with Bearer"))
  }
}

pub trait UserClaimsRequest {
  fn get_user_id(&self) -> AppResult<Uuid>;
  fn get_user_claims(&self) -> AppResult<UserClaims>;
}

impl UserClaimsRequest for HttpRequest {
  fn get_user_id(&self) -> AppResult<Uuid> {
    self
      .extensions()
      .get::<UserClaims>()
      .map(|u| Uuid::parse_str(&u.uid).map_err(|e| e.into()))
      .ok_or_else(|| AppError::Unauthorized("User Must Login".to_string()))?
  }

  fn get_user_claims(&self) -> AppResult<UserClaims> {
    self
      .extensions()
      .get::<UserClaims>()
      .cloned()
      .ok_or_else(|| AppError::Unauthorized("User Must Login".to_string()))
  }
}

#[cfg(test)]
mod tests {
  use actix_web::http::header;
  use actix_web::http::header::HeaderValue;
  use fake::{Fake, Faker};

  use crate::key::RsaPairKey;

  use super::*;

  #[test]
  fn test_user_claims() {
    let user_id: Uuid = Faker.fake();
    let session_id: Uuid = Faker.fake();
    let pair_key = RsaPairKey::new(2048).unwrap();
    let claims = UserClaims::new(
      Duration::from_secs(100),
      &user_id,
      &session_id,
      RoleUser::User,
    );
    // println!(
    //     "private key: {}",
    //     String::from_utf8(pair_key.private_key.clone()).unwrap()
    // );
    // println!(
    //     "public key: {}",
    //     String::from_utf8(pair_key.public_key.clone()).unwrap()
    // );
    let token = claims
      .encode(&EncodingKey::from_rsa_pem(&pair_key.private_key).unwrap())
      .unwrap();
    let actual_claims = UserClaims::decode(
      &token,
      &DecodingKey::from_rsa_pem(&pair_key.public_key).unwrap(),
    )
    .unwrap()
    .claims;
    assert_eq!(actual_claims, claims)
  }

  #[test]
  fn test_parse_valid_bearer_token_from_header() {
    let token: String = Faker.fake();
    let bearer = format!("Bearer {token}");
    let mut headers = HeaderMap::new();
    headers.insert(
      header::AUTHORIZATION,
      HeaderValue::from_str(&bearer).unwrap(),
    );
    let result = parse_bearer_token_from_header(&headers).unwrap();
    assert_eq!(result, token)
  }

  #[test]
  fn test_parse_invalid_bearer_prefix_token() {
    let token: String = Faker.fake();
    let bearer = format!("bearer {token}");
    let mut headers = HeaderMap::new();
    headers.insert(
      header::AUTHORIZATION,
      HeaderValue::from_str(&bearer).unwrap(),
    );
    let result = parse_bearer_token_from_header(&headers);
    assert!(result.is_err(), "result: {result:?}");
    let bearer = format!("b {token}");
    let mut headers = HeaderMap::new();
    headers.insert(
      header::AUTHORIZATION,
      HeaderValue::from_str(&bearer).unwrap(),
    );
    let result = parse_bearer_token_from_header(&headers);
    assert!(result.is_err(), "result: {result:?}");
  }
}
