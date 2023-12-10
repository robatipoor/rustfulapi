use super::http::CLIENT;
use super::result::AppResponseResult;
use log_derive::logfn;
use reqwest::StatusCode;
use rustfulapi::client::http::HttpClientExt;
use rustfulapi::configure::server::ServerConfig;
use rustfulapi::dto::request::*;
use rustfulapi::dto::response::*;
use rustfulapi::dto::ServiceStatusResponse;
use rustfulapi::util::claim::UserClaims;

pub struct Api {
  addr: String,
}

impl Api {
  pub fn new(config: &ServerConfig) -> Self {
    Self {
      addr: config.get_http_addr(),
    }
  }

  #[logfn(Info)]
  pub async fn server_state(
    &self,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<ServiceStatusResponse>)> {
    let resp = CLIENT
      .get_request(&format!("{}/api/v1/server/state", self.addr))
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn health_check(&self) -> anyhow::Result<(StatusCode, AppResponseResult)> {
    let resp = CLIENT
      .get_request(&format!("{}/api/v1/server/health_check", self.addr))
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn register(
    &self,
    req: &RegisterRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<RegisterResponse>)> {
    let resp = CLIENT
      .post_request(&format!("{}/api/v1/users/register", self.addr), req)
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn active(
    &self,
    req: &ActiveRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<MessageResponse>)> {
    let resp = CLIENT
      .put_request(&format!("{}/api/v1/users/active", self.addr), req)
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn login(
    &self,
    req: &LoginRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<LoginResponse>)> {
    let resp = CLIENT
      .post_request(&format!("{}/api/v1/users/login", self.addr), req)
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn login2fa(
    &self,
    req: &Login2faRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<LoginResponse>)> {
    let resp = CLIENT
      .post_request(&format!("{}/api/v1/users/login2fa", self.addr), req)
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn logout(&self, token: &str) -> anyhow::Result<(StatusCode, AppResponseResult)> {
    let resp = CLIENT
      .get(format!("{}/api/v1/users/logout", self.addr))
      .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn refresh_token(
    &self,
    refresh_token: &str,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<TokenResponse>)> {
    let resp = CLIENT
      .get(format!("{}/api/v1/token/refresh", self.addr))
      .header(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {refresh_token}"),
      )
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn forget_password(
    &self,
    email: &str,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<ForgetPasswordResponse>)> {
    let resp = CLIENT
      .get_request(&format!(
        "{}/api/v1/users/password?email={}",
        self.addr, email
      ))
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn reset_password(
    &self,
    req: &SetPasswordRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult)> {
    let resp = CLIENT
      .put_request(&format!("{}/api/v1/users/password", self.addr), req)
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn token_info(
    &self,
    owner_token: &str,
    req: &TokenInfoRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<UserClaims>)> {
    let resp = CLIENT
      .post(format!("{}/api/v1/token/info", self.addr))
      .header(
        reqwest::header::AUTHORIZATION,
        format!("Bearer {owner_token}"),
      )
      .json(req)
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn get_profile(
    &self,
    token: &str,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<ProfileResponse>)> {
    let resp = CLIENT
      .get(format!("{}/api/v1/users/profile", self.addr))
      .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn update_profile(
    &self,
    token: &str,
    req: &UpdateProfileRequest,
  ) -> reqwest::Result<(StatusCode, AppResponseResult)> {
    let resp = CLIENT
      .put(format!("{}/api/v1/users/profile", self.addr))
      .json(req)
      .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
      .send()
      .await?;
    Result::<_, reqwest::Error>::Ok((resp.status(), resp.json().await?))
  }
}
