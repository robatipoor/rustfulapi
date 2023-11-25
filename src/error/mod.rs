use axum::{
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use serde::Deserialize;
use serde::Serialize;
use strum::EnumString;
use utoipa::ToSchema;

use crate::entity;

pub type AppResult<T = ()> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error, ToSchema)]
pub enum AppError {
  #[error("{0} not found")]
  NotFoundError(Resource),
  #[error("{0} not available")]
  NotAvailableError(Resource),
  #[error("{0} already exists")]
  ResourceExistsError(Resource),
  #[error("{0}")]
  PermissionDeniedError(String),
  #[error("{0}")]
  UserBlockedError(String),
  #[error("{0}")]
  UserNotActiveError(String),
  #[error("{0}")]
  InvalidSessionError(String),
  #[error("{0}")]
  ConflictError(String),
  #[error("{0}")]
  UnauthorizedError(String),
  #[error("bad request {0}")]
  BadRequestError(String),
  #[error("{0}")]
  InvalidPayloadError(String),
  #[error("access denied {0}")]
  AccessDeniedError(String),
  #[error("{0}")]
  HashError(String),
  #[error(transparent)]
  InvalidInputError(#[from] garde::Report),
  #[error(transparent)]
  DatabaseError(#[from] sea_orm::error::DbErr),
  #[error(transparent)]
  WebSocketError(#[from] tokio_tungstenite::tungstenite::Error),
  #[error(transparent)]
  IoError(#[from] std::io::Error),
  #[error(transparent)]
  UuidError(#[from] uuid::Error),
  #[error(transparent)]
  JwtError(#[from] jsonwebtoken::errors::Error),
  #[error(transparent)]
  HttpClientError(#[from] reqwest::Error),
  #[error(transparent)]
  RedisError(#[from] redis::RedisError),
  #[error(transparent)]
  ConfigError(#[from] config::ConfigError),
  #[error(transparent)]
  SmtpError(#[from] lettre::transport::smtp::Error),
  #[error(transparent)]
  LetterError(#[from] lettre::error::Error),
  #[error(transparent)]
  ParseJsonError(#[from] serde_json::Error),
  #[error(transparent)]
  ParseFloatError(#[from] std::num::ParseFloatError),
  #[error(transparent)]
  AddrParseError(#[from] std::net::AddrParseError),
  #[error(transparent)]
  SpawnTaskError(#[from] tokio::task::JoinError),
  #[error(transparent)]
  TeraError(#[from] tera::Error),
  #[error(transparent)]
  Base64Error(#[from] base64::DecodeError),
  #[error(transparent)]
  StrumParseError(#[from] strum::ParseError),
  #[error(transparent)]
  SystemTimeError(#[from] std::time::SystemTimeError),
  #[error(transparent)]
  AxumError(#[from] axum::Error),
  #[error(transparent)]
  UnknownError(#[from] anyhow::Error),
  #[error(transparent)]
  Infallible(#[from] std::convert::Infallible),
  #[error(transparent)]
  TypeHeaderError(#[from] axum_extra::typed_header::TypedHeaderRejection),
}

impl From<argon2::password_hash::Error> for AppError {
  fn from(value: argon2::password_hash::Error) -> Self {
    AppError::HashError(value.to_string())
  }
}

impl AppError {
  pub fn response(self) -> (AppResponseError, StatusCode) {
    use AppError::*;
    let (kind, message, code, details, status_code) = match self {
      InvalidPayloadError(err) => (
        "INVALID_PAYLOAD_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::BAD_REQUEST,
      ),
      BadRequestError(err) => (
        "BAD_REQUEST_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::BAD_REQUEST,
      ),
      NotAvailableError(ref resource) => (
        format!("{resource}_NOT_AVAILABLE_ERROR"),
        self.to_string(),
        None,
        vec![],
        StatusCode::NOT_FOUND,
      ),
      NotFoundError(ref resource) => (
        format!("{resource}_NOT_FOUND_ERROR"),
        self.to_string(),
        Some(resource.resource_type as i32),
        resource.details.clone(),
        StatusCode::NOT_FOUND,
      ),
      AccessDeniedError(err) => (
        "ACCESS_DENIED_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      ResourceExistsError(ref resource) => (
        format!("{resource}_ALREADY_EXISTS_ERROR"),
        self.to_string(),
        Some(resource.resource_type as i32),
        resource.details.clone(),
        StatusCode::CONFLICT,
      ),
      AxumError(err) => (
        "AXUM_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      ConfigError(err) => (
        "CONFIG_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      AddrParseError(err) => (
        "ADDR_PARSE_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      IoError(err) => {
        let (status, kind, code) = match err.kind() {
          std::io::ErrorKind::NotFound => (
            StatusCode::NOT_FOUND,
            format!("{}_NOT_FOUND_ERROR", ResourceType::File),
            Some(ResourceType::File as i32),
          ),
          std::io::ErrorKind::PermissionDenied => {
            (StatusCode::FORBIDDEN, "FORBIDDEN_ERROR".to_string(), None)
          }
          _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "IO_ERROR".to_string(),
            None,
          ),
        };
        (kind, err.to_string(), code, vec![], status)
      }
      WebSocketError(err) => (
        "WEBSOCKET_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      ParseJsonError(err) => (
        "PARSE_JSON_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      StrumParseError(err) => (
        "STRUM_PARSE_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      HttpClientError(err) => (
        "HTTP_CLIENT_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      SystemTimeError(err) => (
        "SYSTEM_TIME_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      SpawnTaskError(err) => (
        "SPAWN_TASK_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      UnknownError(err) => (
        "UNKNOWN_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::INTERNAL_SERVER_ERROR,
      ),
      PermissionDeniedError(err) => (
        "PERMISSION_DENIED_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      UserBlockedError(err) => (
        "USER_BLOCKED_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      InvalidSessionError(err) => (
        "INVALID_SESSION_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      ConflictError(err) => (
        "CONFLICT_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      UserNotActiveError(err) => (
        "USER_NOT_ACTIVE_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      UnauthorizedError(err) => (
        "UNAUTHORIZED_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      UuidError(err) => (
        "UUID_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      JwtError(err) => (
        "JWT_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      RedisError(err) => (
        "REDIS_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      SmtpError(err) => (
        "SMTP_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      LetterError(err) => (
        "LETTER_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      HashError(err) => (
        "HASH_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      ParseFloatError(err) => (
        "PARSE_FLOAT_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      TeraError(err) => (
        "TERA_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      Base64Error(err) => (
        "BASE64_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      InvalidInputError(err) => (
        "INVALID_INPUT_ERROR".to_string(),
        "The input is invalid.".to_string(),
        None,
        err
          .iter()
          .map(|(p, e)| (p.to_string(), e.to_string()))
          .collect(),
        StatusCode::FORBIDDEN,
      ),
      DatabaseError(err) => (
        "DATABASE_ERROR".to_string(),
        err.to_string(),
        None,
        vec![],
        StatusCode::FORBIDDEN,
      ),
      Infallible(_) => todo!(),
      TypeHeaderError(_) => todo!(),
    };

    (
      AppResponseError::new(kind, message, code, details),
      status_code,
    )
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    let (body, status_code) = self.response();
    (status_code, Json(body)).into_response()
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "type", rename = "AppError")]
pub struct AppResponseError {
  pub kind: String,
  pub message: String,
  pub code: Option<i32>,
  pub details: Vec<(String, String)>,
}

impl AppResponseError {
  pub fn new(
    kind: impl Into<String>,
    message: impl Into<String>,
    code: Option<i32>,
    details: Vec<(String, String)>,
  ) -> Self {
    Self {
      kind: kind.into(),
      message: message.into(),
      code,
      details,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Resource {
  pub details: Vec<(String, String)>,
  pub resource_type: ResourceType,
}

impl std::fmt::Display for Resource {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // TODO
    self.resource_type.fmt(f)
  }
}

#[derive(Debug, EnumString, strum::Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceType {
  #[strum(serialize = "USER")]
  User,
  #[strum(serialize = "FILE")]
  File,
  #[strum(serialize = "SESSION")]
  Session,
  #[strum(serialize = "MESSAGE")]
  Message,
}

pub fn invalid_input_error(field: &'static str, message: &'static str) -> AppError {
  let mut report = garde::Report::new();
  report.append(garde::Path::new(field), garde::Error::new(message));
  AppError::InvalidInputError(report)
}

pub trait ToAppResult {
  type Output: entity::AppEntity;
  fn to_result(self) -> AppResult<Self::Output>;
  fn check_absent(self) -> AppResult;
  fn check_absent_details(self, details: Vec<(String, String)>) -> AppResult;
  fn to_result_details(self, details: Vec<(String, String)>) -> AppResult<Self::Output>;
}

impl<T> ToAppResult for Option<T>
where
  T: entity::AppEntity,
{
  type Output = T;
  fn to_result(self) -> AppResult<Self::Output> {
    self.ok_or_else(|| {
      AppError::NotFoundError(Resource {
        details: vec![],
        resource_type: Self::Output::RESOURCE,
      })
    })
  }

  fn to_result_details(self, details: Vec<(String, String)>) -> AppResult<Self::Output> {
    self.ok_or_else(|| {
      AppError::NotFoundError(Resource {
        details,
        resource_type: Self::Output::RESOURCE,
      })
    })
  }

  fn check_absent(self) -> AppResult {
    if self.is_some() {
      Err(AppError::ResourceExistsError(Resource {
        details: vec![],
        resource_type: Self::Output::RESOURCE,
      }))
    } else {
      Ok(())
    }
  }

  fn check_absent_details(self, details: Vec<(String, String)>) -> AppResult {
    if self.is_some() {
      Err(AppError::ResourceExistsError(Resource {
        details,
        resource_type: Self::Output::RESOURCE,
      }))
    } else {
      Ok(())
    }
  }
}
