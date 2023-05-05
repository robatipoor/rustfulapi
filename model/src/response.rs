use chrono::{DateTime, Utc};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use constant::BEARER;
use entity::*;

#[derive(Debug, Serialize, Deserialize, ToSchema, Dummy, Clone)]
pub struct SaveUserResponse {
  pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Dummy, Clone)]
pub struct GetUserResponse {
  pub id: Uuid,
  pub username: String,
  pub email: String,
  pub role_name: RoleUser,
  pub is_active: bool,
  pub is_tfa: bool,
  pub create_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Dummy, Clone)]
pub struct ServiceStatusResponse {
  pub postgres: bool,
  pub redis: bool,
  pub email: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
  pub message: String,
}

impl MessageResponse {
  pub fn new<S: Into<String>>(message: S) -> Self {
    Self {
      message: message.into(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct PageResponse<T> {
  pub data: Vec<T>,
  pub page_num: i64,
  pub page_size: i64,
  pub total: i64,
}

impl<T> PageResponse<T> {
  pub fn new(data: Vec<T>, page_num: i64, page_size: i64, total: i64) -> PageResponse<T> {
    PageResponse {
      data,
      page_num,
      page_size,
      total,
    }
  }

  pub fn map<F, B>(&self, f: F) -> PageResponse<B>
  where
    F: FnMut(&T) -> B,
  {
    let data: Vec<B> = self.data.iter().map(f).collect();
    PageResponse {
      data,
      page_num: self.page_num,
      page_size: self.page_size,
      total: self.total,
    }
  }
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Dummy)]
pub struct RegisterResponse {
  pub id: Uuid,
  pub expire_in: u64,
}

impl RegisterResponse {
  pub fn new(id: Uuid, expire_in: u64) -> Self {
    Self { id, expire_in }
  }
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Dummy)]
pub struct InvitationResponse {
  pub id: Uuid,
  pub expire_in: u64,
}

impl InvitationResponse {
  pub fn new(id: Uuid, expire_in: u64) -> Self {
    Self { id, expire_in }
  }
}
// TODO
#[derive(Debug, Deserialize, Serialize, ToSchema, Dummy)]
#[serde(tag = "type")]
pub enum LoginResponse {
  Token {
    token_type: String,
    access_token: String,
    refresh_token: String,
    expire_in: u64,
  },
  Id {
    // expire_in: u64,
    id: Uuid,
  },
}

impl From<TokenResponse> for LoginResponse {
  fn from(value: TokenResponse) -> Self {
    LoginResponse::Token {
      token_type: value.token_type,
      access_token: value.access_token,
      refresh_token: value.refresh_token,
      expire_in: value.expire_in,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Dummy, Clone)]
pub struct TokenResponse {
  pub token_type: String,
  pub access_token: String,
  pub refresh_token: String,
  pub expire_in: u64,
}

impl TokenResponse {
  pub fn new(access_token: String, refresh_token: String, expire_in: u64) -> Self {
    Self {
      token_type: BEARER.to_string(),
      access_token,
      refresh_token,
      expire_in,
    }
  }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Dummy)]
pub struct ForgetPasswordResponse {
  // expire_in: u64,
  pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ProfileResponse {
  pub username: String,
}

impl From<&User> for ProfileResponse {
  fn from(user: &User) -> Self {
    ProfileResponse {
      username: user.username.clone(),
    }
  }
}
