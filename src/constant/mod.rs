use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use sea_orm::Database;
use std::{path::PathBuf, time::Duration};

use crate::{
  client::{
    database::{DatabaseClient, DatabaseClientExt},
    email::EmailClient,
    http::HttpClient,
    redis::RedisClient,
    ClientBuilder,
  },
  error::AppResult,
  util,
};

// if you change the token length you most change validate request length
pub const VERIFY_CODE_LEN: usize = 5;
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(120);
pub const EXPIRE_SESSION_CODE_SECS: Duration = Duration::from_secs(2000);
pub const EXPIRE_INVITATION_CODE_SECS: Duration = Duration::from_secs(86000);
pub const EXPIRE_BLOCKED_EMAIL_SECS: Duration = Duration::from_secs(100);
pub const EXPIRE_FORGET_PASS_CODE_SECS: Duration = Duration::from_secs(200);
pub const EXPIRE_TWO_FACTOR_CODE_SECS: Duration = Duration::from_secs(200);
pub const EXPIRE_BEARER_TOKEN_SECS: Duration = Duration::from_secs(600);
pub const EXPIRE_REFRESH_TOKEN_SECS: Duration = Duration::from_secs(3600);
pub const QUEUE_EMPTY_DELAY_SECS: Duration = Duration::from_secs(60);
pub const COMPLETE_TASK_DELAY_SECS: Duration = Duration::from_secs(10);
pub const REFRESH_TOKEN_ROUTE: &str = "/api/v1/users/token";
pub const IGNORE_ROUTES: [&str; 8] = [
  "/api/v1/server/health_check",
  "/api/v1/server/state",
  "/api/v1/users/register",
  "/api/v1/users/active",
  "/api/v1/users/login",
  "/api/v1/users/password",
  "/api/v1/swagger-ui",
  "/api/v1/api-doc",
];
pub const AUTHORIZATION: &str = "Authorization";
pub const BEARER: &str = "Bearer";
pub const APP_DOMAIN: &str = "rustfulapi.com";
pub const APP_EMAIL_ADDR: &str = "rustfulapi@email.com";
pub static IMAGES_PATH: Lazy<PathBuf> = Lazy::new(|| util::dir::root_dir("static/images").unwrap());
pub static APP_IMAGE: Lazy<PathBuf> =
  Lazy::new(|| util::dir::root_dir("static/images/logo.jpg").unwrap());
pub static CONFIG: Lazy<crate::configure::AppConfig> =
  Lazy::new(|| crate::configure::AppConfig::read().unwrap());
pub static HTTP: Lazy<reqwest::Client> =
  Lazy::new(|| HttpClient::build_from_config(&CONFIG).unwrap());
pub static REDIS: Lazy<RedisClient> =
  Lazy::new(|| RedisClient::build_from_config(&CONFIG).unwrap());
pub static EMAIL: Lazy<EmailClient> =
  Lazy::new(|| EmailClient::build_from_config(&CONFIG).unwrap());
pub const MAX_RETRY: u32 = 10;
pub const MINIMUM_DELAY_TIME: std::time::Duration = std::time::Duration::from_millis(100);
pub static ENCODE_KEY: Lazy<EncodingKey> = Lazy::new(|| {
  let key = CONFIG.secret.read_private_refresh_key().unwrap();
  EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
pub static DECODE_KEY: Lazy<DecodingKey> = Lazy::new(|| {
  let key = CONFIG.secret.read_public_refresh_key().unwrap();
  DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});

static DATABASE: tokio::sync::OnceCell<DatabaseClient> = tokio::sync::OnceCell::const_new();
pub async fn get_database() -> AppResult<&'static DatabaseClient> {
  DATABASE
    .get_or_try_init(|| async { DatabaseClient::build_from_config(&CONFIG).await })
    .await
}
