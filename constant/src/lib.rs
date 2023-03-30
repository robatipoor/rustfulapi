use once_cell::sync::Lazy;
use std::{
  path::{Path, PathBuf},
  time::Duration,
};

// if you change the token length you most change validate request length
pub const VERIFY_CODE_LEN: usize = 5;
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
pub static IMAGES_PATH: Lazy<PathBuf> = Lazy::new(|| root_dir("static/images").unwrap());
pub static APP_IMAGE: Lazy<PathBuf> = Lazy::new(|| root_dir("static/images/logo.jpg").unwrap());

pub fn root_dir<P: AsRef<Path>>(path: P) -> std::io::Result<PathBuf> {
  Ok(
    project_root::get_project_root()
      .or_else(|_| std::env::current_dir())?
      .join(path),
  )
}
