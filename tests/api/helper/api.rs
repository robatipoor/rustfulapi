use crate::unwrap;

use super::result::AppResponseResult;
use log_derive::logfn;
use reqwest::StatusCode;
use rustfulapi::client::http::HttpClientExt;
use rustfulapi::configure::server::ServerConfig;
use rustfulapi::constant::HTTP;
use rustfulapi::dto::request::*;
use rustfulapi::dto::response::*;
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
    let resp = HTTP
      .get_request(&format!("{}/api/v1/server/state", self.addr))
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn health_check(&self) -> anyhow::Result<(StatusCode, AppResponseResult)> {
    let resp = HTTP
      .get_request(&format!("{}/api/v1/server/health_check", self.addr))
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn register(
    &self,
    req: &RegisterRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<RegisterResponse>)> {
    let resp = HTTP
      .post_request(&format!("{}/api/v1/user/register", self.addr), req)
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn active(
    &self,
    req: &ActiveRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<MessageResponse>)> {
    let resp = HTTP
      .put_request(&format!("{}/api/v1/user/active", self.addr), req)
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn login(
    &self,
    req: &LoginRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<LoginResponse>)> {
    let resp = HTTP
      .post_request(&format!("{}/api/v1/user/login", self.addr), req)
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn get_token(&self, req: &LoginRequest) -> anyhow::Result<TokenResponse> {
    let (_, resp) = self.login(req).await?;
    let resp = unwrap!(resp);
    match resp {
      LoginResponse::Token(token) => Ok(token),
      LoginResponse::Code { .. } => Err(anyhow::anyhow!("Get token failed.")),
    }
  }

  #[logfn(Info)]
  pub async fn get_user_list(
    &self,
    param: &PageQueryParam,
    token: &str,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<GetUserListResponse>)> {
    let resp = HTTP
      .get(&format!("{}/api/v1/admin/user/list", self.addr))
      .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
      .query(param)
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn login2fa(
    &self,
    req: &Login2faRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<LoginResponse>)> {
    let resp = HTTP
      .post_request(&format!("{}/api/v1/user/login2fa", self.addr), req)
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn logout(&self, token: &str) -> anyhow::Result<(StatusCode, AppResponseResult)> {
    let resp = HTTP
      .get(format!("{}/api/v1/user/logout", self.addr))
      .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn refresh_token(
    &self,
    req: &RefreshTokenRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<TokenResponse>)> {
    let resp = HTTP
      .post(format!("{}/api/v1/token/refresh", self.addr))
      .json(req)
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn forget_password(
    &self,
    email: &str,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<ForgetPasswordResponse>)> {
    let resp = HTTP
      .get_request(&format!(
        "{}/api/v1/user/password?email={}",
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
    let resp = HTTP
      .put_request(&format!("{}/api/v1/user/password", self.addr), req)
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn token_info(
    &self,
    owner_token: &str,
    req: &TokenInfoRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<UserClaims>)> {
    let resp = HTTP
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
    let resp = HTTP
      .get(format!("{}/api/v1/user/profile", self.addr))
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
    let resp = HTTP
      .put(format!("{}/api/v1/user/profile", self.addr))
      .json(req)
      .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
      .send()
      .await?;
    Result::<_, reqwest::Error>::Ok((resp.status(), resp.json().await?))
  }
}
