use fake::faker::internet::en::{Password, SafeEmail, Username};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Dummy, Validate, ToSchema)]
pub struct RegisterRequest {
  #[dummy(faker = "Username()")]
  pub username: String,
  #[dummy(faker = "SafeEmail()")]
  #[validate(email)]
  pub email: String,
  #[dummy(faker = "Password(8..100)")]
  #[validate(length(min = 8))]
  pub password: String,
}

impl RegisterRequest {
  pub fn new(username: &str, email: &str, password: &str) -> Self {
    Self {
      password: password.to_string(),
      username: username.to_string(),
      email: email.to_string(),
    }
  }

  pub fn to_json(&self) -> Result<String, serde_json::Error> {
    serde_json::to_string(&self)
  }
}

#[derive(Debug, Deserialize, Serialize, Dummy, Validate, ToSchema)]
pub struct InvitationRequest {
  #[dummy(faker = "SafeEmail()")]
  #[validate(email)]
  pub email: String,
  #[dummy(faker = "Password(8..100)")]
  #[validate(length(min = 8))]
  pub password: String,
}

impl InvitationRequest {
  pub fn new(email: String, password: String) -> Self {
    Self { email, password }
  }
}

#[derive(Debug, Deserialize, Serialize, Dummy, Validate, ToSchema, IntoParams, Clone)]
pub struct PageParamQuery {
  pub page_num: i64,
  pub page_size: i64,
  pub sort_by: Option<String>,
  pub sort_direction: Option<Direction>,
}

#[derive(Serialize, Deserialize, Debug, Display, Dummy, ToSchema, Clone, Copy)]
pub enum Direction {
  DESC,
  ASC,
}

// TODO #![feature(unboxed_closures)] unstable
impl Direction {
  pub fn as_closure<T>(&self) -> impl Fn((T, T)) -> bool
  where
    T: Ord,
  {
    match self {
      Direction::ASC => |(a, b)| a <= b,
      Direction::DESC => |(a, b)| a >= b,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Dummy, Validate, ToSchema, IntoParams)]
pub struct ActiveRequest {
  #[validate(length(min = 5))]
  pub code: String,
  pub id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Dummy, ToSchema)]
#[serde(tag = "type")]
pub enum LoginRequest {
  Normal(NormalLogin),
  TwoFactor(TwoFactorLogin),
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate, Dummy)]
pub struct NormalLogin {
  #[dummy(faker = "SafeEmail()")]
  #[validate(email)]
  pub email: String,
  #[dummy(faker = "Password(8..100)")]
  #[validate(length(min = 8))]
  pub password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate, Dummy)]
pub struct TwoFactorLogin {
  pub id: Uuid,
  #[validate(length(min = 5))]
  pub code: String,
}

impl Validate for LoginRequest {
  fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
    let errors = validator::ValidationErrors::new();
    let result = if errors.is_empty() {
      Ok(())
    } else {
      Err(errors)
    };
    match self {
      Self::Normal(n) => validator::ValidationErrors::merge(result, "Normal", n.validate()),
      Self::TwoFactor(t) => validator::ValidationErrors::merge(result, "TwoFactor", t.validate()),
    }
  }
}

#[derive(Debug, Deserialize, ToSchema, Validate, Dummy, IntoParams)]
pub struct TwoFactorLoginRequest {
  #[dummy(faker = "SafeEmail()")]
  #[validate(email)]
  pub email: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate, Dummy, IntoParams)]
pub struct RefreshTokenRequest {
  #[validate(length(equal = 30))]
  pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, Dummy, IntoParams)]
pub struct ValidateRequest {
  pub token: String,
}
#[derive(Debug, Deserialize, ToSchema, Validate, Dummy, IntoParams)]
pub struct ForgetPasswordParamQuery {
  #[dummy(faker = "SafeEmail()")]
  #[validate(email)]
  pub email: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate, Dummy, IntoParams)]
pub struct SetPasswordRequest {
  #[dummy(faker = "Password(8..100)")]
  #[validate(length(min = 8))]
  pub new_password: String,
  #[validate(length(min = 5))]
  pub code: String,
  pub id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate, Dummy, Default)]
pub struct UpdateProfileRequest {
  #[dummy(faker = "Username()")]
  pub username: Option<String>,
  #[dummy(faker = "Password(8..100)")]
  #[validate(length(min = 8))]
  pub password: Option<String>,
  pub is_tfa: Option<bool>,
  pub is_private: Option<bool>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_invalid_email_register_request() {
    let req = RegisterRequest::new("username", "email", "password");
    assert!(req.validate().is_err());
  }

  #[test]
  fn test_invalid_pass_register_request() {
    let req = RegisterRequest::new("username", "email@test.com", "pass");
    assert!(req.validate().is_err());
  }

  #[test]
  fn test_valid_user_register_request() {
    let req = RegisterRequest::new("foo", "foo@bar.com", "password");
    assert!(req.validate().is_ok());
  }

  #[test]
  fn test_valid_register_request() {
    let req = RegisterRequest::new("username", "email@test.com", "password");
    assert!(req.validate().is_ok());
  }
}
