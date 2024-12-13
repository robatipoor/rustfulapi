use jsonwebtoken::{DecodingKey, EncodingKey};
use std::{path::PathBuf, sync::LazyLock, time::Duration};
use utoipa::OpenApi;

use crate::{
  client::{email::EmailClient, http::HttpClient, redis::RedisClient, ClientBuilder},
  configure::{env::get_env_source, get_static_dir, template::TemplateEngine},
  handler::openapi::ApiDoc,
};

pub const ENV_PREFIX: &str = "APP";
pub const CODE_LEN: usize = 5;
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(120);
pub const EXPIRE_SESSION_CODE_SECS: Duration = Duration::from_secs(2000);
pub const EXPIRE_INVITATION_CODE_SECS: Duration = Duration::from_secs(86000);
pub const EXPIRE_BLOCKED_EMAIL_SECS: Duration = Duration::from_secs(100);
pub const EXPIRE_FORGET_PASS_CODE_SECS: Duration = Duration::from_secs(100);
pub const EXPIRE_TWO_FACTOR_CODE_SECS: Duration = Duration::from_secs(200);
pub const EXPIRE_BEARER_TOKEN_SECS: Duration = Duration::from_secs(600);
pub const EXPIRE_REFRESH_TOKEN_SECS: Duration = Duration::from_secs(3600);
pub const QUEUE_EMPTY_DELAY_SECS: Duration = Duration::from_secs(60);
pub const COMPLETE_TASK_DELAY_SECS: Duration = Duration::from_secs(10);
pub const CHECK_EMAIL_MESSAGE: &str = "Please check you email.";
pub const AUTHORIZATION: &str = "Authorization";
pub const BEARER: &str = "Bearer";
pub const APP_DOMAIN: &str = "rustfulapi.com";
pub const APP_EMAIL_ADDR: &str = "rustfulapi@email.com";
pub static IMAGES_PATH: LazyLock<PathBuf> =
  LazyLock::new(|| get_static_dir().unwrap().join("images"));
pub static APP_IMAGE: LazyLock<PathBuf> =
  LazyLock::new(|| get_static_dir().unwrap().join("images/logo.jpg"));
pub static CONFIG: LazyLock<crate::configure::AppConfig> =
  LazyLock::new(|| crate::configure::AppConfig::read(get_env_source(ENV_PREFIX)).unwrap());
pub static HTTP: LazyLock<reqwest::Client> =
  LazyLock::new(|| HttpClient::build_from_config(&CONFIG).unwrap());
pub static REDIS: LazyLock<RedisClient> =
  LazyLock::new(|| RedisClient::build_from_config(&CONFIG).unwrap());
pub static EMAIL: LazyLock<EmailClient> =
  LazyLock::new(|| EmailClient::build_from_config(&CONFIG).unwrap());
pub const MAX_RETRY: u32 = 10;
pub const MINIMUM_DELAY_TIME: std::time::Duration = std::time::Duration::from_millis(100);
pub static REFRESH_TOKEN_ENCODE_KEY: LazyLock<EncodingKey> = LazyLock::new(|| {
  let key = CONFIG.secret.read_private_refresh_key().unwrap();
  EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
pub static REFRESH_TOKEN_DECODE_KEY: LazyLock<DecodingKey> = LazyLock::new(|| {
  let key = CONFIG.secret.read_public_refresh_key().unwrap();
  DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
pub static ACCESS_TOKEN_ENCODE_KEY: LazyLock<EncodingKey> = LazyLock::new(|| {
  let key = CONFIG.secret.read_private_access_key().unwrap();
  EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
pub static ACCESS_TOKEN_DECODE_KEY: LazyLock<DecodingKey> = LazyLock::new(|| {
  let key = CONFIG.secret.read_public_access_key().unwrap();
  DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
pub static API_DOC: LazyLock<utoipa::openapi::OpenApi> = LazyLock::new(ApiDoc::openapi);
pub static TEMPLATE_ENGIN: LazyLock<TemplateEngine> = LazyLock::new(|| {
  let path = get_static_dir()
    .unwrap()
    .join("template/**/*")
    .into_os_string()
    .into_string()
    .unwrap();
  TemplateEngine::new(&path).unwrap()
});
