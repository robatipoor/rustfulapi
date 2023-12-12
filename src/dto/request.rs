use fake::faker::internet::en::{Password, SafeEmail, Username};
use fake::Dummy;
use garde::Validate;
use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Dummy, Validate, utoipa::ToSchema)]
pub struct RegisterRequest {
  #[dummy(faker = "Username()")]
  #[garde(ascii, length(min = 3, max = 25))]
  pub username: String,
  #[dummy(faker = "SafeEmail()")]
  #[garde(email)]
  pub email: String,
  #[dummy(faker = "Password(8..100)")]
  #[garde(length(min = 8))]
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

#[derive(Debug, Deserialize, Serialize, Dummy, ToSchema, IntoParams, Clone)]
pub struct PageQueryParam {
  pub page_num: u64,
  pub page_size: u64,
  pub sort_by: Option<String>,
  pub sort_direction: Option<Direction>,
}

#[derive(
  Serialize,
  Deserialize,
  Debug,
  Display,
  Dummy,
  ToSchema,
  Clone,
  Copy,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
)]
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

#[derive(Debug, Serialize, Deserialize, Validate, Dummy, ToSchema, IntoParams)]
pub struct ActiveRequest {
  #[garde(length(min = 5))]
  pub code: String,
  #[garde(skip)]
  pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Dummy, ToSchema, Validate)]
#[serde(tag = "type")]
pub struct LoginRequest {
  #[dummy(faker = "SafeEmail()")]
  #[garde(email)]
  pub email: String,
  #[dummy(faker = "Password(8..100)")]
  #[garde(length(min = 8))]
  pub password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate, Dummy)]
pub struct Login2faRequest {
  #[garde(skip)]
  pub user_id: Uuid,
  #[garde(length(min = 5))]
  pub code: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, Dummy, IntoParams)]
pub struct RefreshTokenRequest {
  #[garde(length(min = 30))]
  pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, Dummy, IntoParams)]
pub struct TokenInfoRequest {
  #[garde(length(min = 30))]
  pub token: String,
}
#[derive(Debug, Deserialize, ToSchema, Validate, Dummy, IntoParams)]
pub struct ForgetPasswordQueryParam {
  #[dummy(faker = "SafeEmail()")]
  #[garde(email)]
  pub email: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate, Dummy, IntoParams)]
pub struct SetPasswordRequest {
  #[dummy(faker = "Password(8..100)")]
  #[garde(length(min = 8))]
  pub new_password: String,
  #[garde(length(min = 5))]
  pub code: String,
  #[garde(skip)]
  pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate, Dummy, Default)]
pub struct UpdateProfileRequest {
  #[dummy(faker = "Username()")]
  #[garde(skip)]
  pub username: Option<String>,
  #[dummy(faker = "Password(8..100)")]
  #[garde(length(min = 8))]
  pub password: Option<String>,
  #[garde(skip)]
  pub is_2fa: Option<bool>,
  #[garde(skip)]
  pub is_private: Option<bool>,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_invalid_email_register_request() {
    let req = RegisterRequest::new("username", "email", "password");
    assert!(req.validate(&()).is_err());
  }

  #[test]
  fn test_invalid_pass_register_request() {
    let req = RegisterRequest::new("username", "email@test.com", "pass");
    assert!(req.validate(&()).is_err());
  }

  #[test]
  fn test_valid_user_register_request() {
    let req = RegisterRequest::new("foo", "foo@bar.com", "password");
    assert!(req.validate(&()).is_ok());
  }

  #[test]
  fn test_valid_register_request() {
    let req = RegisterRequest::new("username", "email@test.com", "password");
    assert!(req.validate(&()).is_ok());
  }
}
