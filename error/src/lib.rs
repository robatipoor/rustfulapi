use std::num::ParseFloatError;

use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use validator::ValidationErrors;

pub type AppResult<T = ()> = std::result::Result<T, AppError>;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AppResponseError {
  pub error: String,
  pub detail: String,
}

impl AppResponseError {
  pub fn new<S: Into<String>>(error: S, detail: S) -> Self {
    Self {
      error: error.into(),
      detail: detail.into(),
    }
  }
}

#[derive(Debug, thiserror::Error, ToSchema)]
pub enum AppError {
  #[error(transparent)]
  InvalidInput(#[from] validator::ValidationErrors),
  #[error("{0}")]
  NotFound(String),
  #[error("{0}")]
  PermissionDenied(String),
  #[error("{0}")]
  UserBlocked(String),
  #[error("{0}")]
  AlreadyExists(String),
  #[error("{0}")]
  InvalidSession(String),
  #[error("{0}")]
  SessionNotExist(String),
  #[error("{0}")]
  Conflict(String),
  #[error("{0}")]
  UserNotActive(String),
  #[error("{0}")]
  Unauthorized(String),
  #[error(transparent)]
  IoError(#[from] std::io::Error),
  #[error(transparent)]
  UuidError(#[from] uuid::Error),
  #[error(transparent)]
  JwtError(#[from] jsonwebtoken::errors::Error),
  #[error(transparent)]
  DatabaseError(#[from] sqlx::Error),
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
  #[error("{0}")]
  HashError(String),
  #[error(transparent)]
  SerdeError(#[from] serde_json::Error),
  #[error(transparent)]
  ParseFloatError(#[from] ParseFloatError),
  #[error(transparent)]
  SpawnTaskError(#[from] tokio::task::JoinError),
  #[error(transparent)]
  TeraError(#[from] tera::Error),
  #[error(transparent)]
  MigrateError(#[from] sqlx::migrate::MigrateError),
  #[error(transparent)]
  UnknownError(#[from] anyhow::Error),
  #[error(transparent)]
  Base64Error(#[from] base64::DecodeError),
  #[error(transparent)]
  MultipartError(#[from] actix_multipart::MultipartError),
}

impl From<argon2::password_hash::Error> for AppError {
  fn from(value: argon2::password_hash::Error) -> Self {
    AppError::HashError(value.to_string())
  }
}

impl AppError {
  pub fn response(&self) -> AppResponseError {
    AppResponseError {
      error: self.error().to_string(),
      detail: self.to_string(),
    }
  }

  pub fn error(&self) -> &'static str {
    match self {
      Self::NotFound(_) => "NOT_FOUND",
      Self::PermissionDenied(_) => "PERMISSION_DENIED",
      Self::UserBlocked(_) => "USER_BLOCKED",
      Self::AlreadyExists(_) => "ALREADY_EXISTS",
      Self::SessionNotExist(_) => "SESSION_NOT_EXIST",
      Self::InvalidSession(_) => "INVALID_SESSION",
      Self::Conflict(_) => "CONFLICT",
      Self::Unauthorized(_) => "UNAUTHORIZED",
      Self::UserNotActive(_) => "USER_NOT_ACTIVE",
      Self::InvalidInput(_) => "INVALID_INPUT",
      Self::MigrateError(err) => {
        tracing::error!("migrate error details: {err}");
        "MIGRATE_ERROR"
      }
      Self::IoError(err) => {
        tracing::error!("io error details: {err}");
        "IO_ERROR"
      }
      Self::ConfigError(err) => {
        tracing::error!("config error details: {err}");
        "CONFIG_ERROR"
      }
      Self::RedisError(err) => {
        tracing::error!("redis error details: {err}");
        "REDIS_ERROR"
      }
      Self::JwtError(err) => {
        tracing::error!("jwt error details: {err}");
        "JWT_ERROR"
      }
      Self::UuidError(err) => {
        tracing::error!("uuid error details: {err}");
        "UUID_ERROR"
      }
      Self::SmtpError(err) => {
        tracing::error!("smtp error details: {err}");
        "SMTP_ERROR"
      }
      Self::HashError(err) => {
        tracing::error!("hash error details: {err}");
        "HASH_ERROR"
      }
      Self::TeraError(err) => {
        tracing::error!("tera error details: {err}");
        "TERA_ERROR"
      }
      Self::SerdeError(err) => {
        tracing::error!("serde error details: {err}");
        "SERDE_ERROR"
      }
      Self::ParseFloatError(err) => {
        tracing::error!("parse float number error details: {err}");
        "PARSE_FLOAT_ERROR"
      }
      Self::HttpClientError(err) => {
        tracing::error!("reqwest error details: {err}");
        "HTTP_CLIENT_ERROR"
      }
      Self::DatabaseError(err) => {
        tracing::error!("sqlx error details: {err}");
        "DATABASE_ERROR"
      }
      Self::SpawnTaskError(err) => {
        tracing::error!("spawn task error details: {err}");
        "SPAWN_TASK_ERROR"
      }
      Self::LetterError(err) => {
        tracing::error!("letter error details: {err}");
        "LETTER_ERROR"
      }
      Self::Base64Error(err) => {
        tracing::error!("base64 error details: {err}");
        "BASE64_ERROR"
      }
      Self::MultipartError(err) => {
        tracing::error!("multipart error details: {err}");
        "MULTIPART_ERROR"
      }
      Self::UnknownError(err) => {
        tracing::error!("unknown error details: {err}");
        "INTERNAL_SERVER_ERROR"
      }
    }
  }

  pub fn status_code(&self) -> StatusCode {
    match self {
      Self::NotFound(_) => StatusCode::NOT_FOUND,
      Self::PermissionDenied(_) | Self::UserBlocked(_) => StatusCode::FORBIDDEN,
      Self::AlreadyExists(_) => StatusCode::from_u16(403).unwrap(),
      Self::Conflict(_) | Self::UserNotActive(_) => StatusCode::from_u16(409).unwrap(), // Conflict
      Self::InvalidInput(_) => StatusCode::from_u16(422).unwrap(), //Unprocessable Entity
      Self::Unauthorized(_)
      | Self::JwtError(_)
      | Self::InvalidSession(_)
      | Self::SessionNotExist(_) => StatusCode::UNAUTHORIZED,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

impl ResponseError for AppError {
  fn status_code(&self) -> StatusCode {
    self.status_code()
  }
  fn error_response(&self) -> HttpResponse {
    let status_code = self.status_code();
    HttpResponse::build(status_code).json(self.response())
  }
}

pub fn invalid_input_error(field: &'static str, message: &'static str) -> AppError {
  let mut err = ValidationErrors::new();
  err.add(
    field,
    validator::ValidationError {
      code: std::borrow::Cow::from("1"),
      message: Some(std::borrow::Cow::Borrowed(message)),
      params: std::collections::HashMap::new(),
    },
  );
  AppError::InvalidInput(err)
}

pub async fn permission_denied_error() -> AppResult<HttpResponse> {
  AppResult::Err(AppError::PermissionDenied(
    "This User Does Not Have Access".to_string(),
  ))
}

#[derive(Debug, thiserror::Error, ToSchema)]
pub enum TaskError {
  #[error(transparent)]
  IoError(#[from] std::io::Error),
  #[error(transparent)]
  DatabaseError(#[from] sqlx::Error),
  #[error(transparent)]
  HttpClientError(#[from] reqwest::Error),
  #[error(transparent)]
  RedisError(#[from] redis::RedisError),
  #[error(transparent)]
  ConfigError(#[from] config::ConfigError),
  #[error(transparent)]
  SpawnTaskError(#[from] tokio::task::JoinError),
}

pub trait ToAppResult<T> {
  fn to_result(self) -> AppResult<T>;
}

impl<T> ToAppResult<T> for Option<T> {
  fn to_result(self) -> AppResult<T> {
    self.ok_or_else(|| AppError::NotFound(format!("{} not found", std::any::type_name::<T>())))
  }
}
