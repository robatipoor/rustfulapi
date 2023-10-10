use actix_web::http;
use configure::server::ServerConfig;
use log_derive::logfn;
use model::request::*;
use model::response::*;
use once_cell::sync::Lazy;
use reqwest::{Client, StatusCode};
use util::claim::UserClaims;

use super::http::CLIENT;
use super::result::AppResponseResult;

pub struct Api {
  client: &'static Client,
  addr: String,
}

impl Api {
  pub fn new(config: &ServerConfig) -> Self {
    Self {
      client: Lazy::force(&CLIENT),
      addr: config.get_http_addr(),
    }
  }

  #[logfn(Info)]
  pub async fn server_state(
    &self,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<ServiceStatusResponse>)> {
    let resp = self
      .client
      .get(format!("{}/api/v1/server/state", self.addr))
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn health_check(&self) -> anyhow::Result<(StatusCode, AppResponseResult)> {
    let resp = self
      .client
      .get(format!("{}/api/v1/server/health_check", self.addr))
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn register(
    &self,
    req: &RegisterRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<RegisterResponse>)> {
    let resp = self
      .client
      .post(format!("{}/api/v1/users/register", self.addr))
      .json(req)
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn invitation(
    &self,
    req: &InvitationRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<InvitationResponse>)> {
    let resp = self
      .client
      .put(format!("{}/api/v1/users/invitation", self.addr))
      .json(req)
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn active(
    &self,
    req: &ActiveRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<MessageResponse>)> {
    let resp = self
      .client
      .put(format!("{}/api/v1/users/active", self.addr))
      .json(req)
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn login(
    &self,
    req: &LoginRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<LoginResponse>)> {
    let resp = self
      .client
      .post(format!("{}/api/v1/users/login", self.addr))
      .json(req)
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn logout(&self, token: &str) -> anyhow::Result<(StatusCode, AppResponseResult)> {
    let resp = self
      .client
      .get(format!("{}/api/v1/users/logout", self.addr))
      .header(http::header::AUTHORIZATION, format!("Bearer {token}"))
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn refresh_token(
    &self,
    refresh_token: &str,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<TokenResponse>)> {
    let resp = self
      .client
      .get(format!("{}/api/v1/users/token", self.addr))
      .header(
        http::header::AUTHORIZATION,
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
    let resp = self
      .client
      .get(format!(
        "{}/api/v1/users/password?email={}",
        self.addr, email
      ))
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn reset_password(
    &self,
    req: &SetPasswordRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult)> {
    let resp = self
      .client
      .put(format!("{}/api/v1/users/password", self.addr))
      .json(req)
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn validate(
    &self,
    owner_token: &str,
    req: &ValidateRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult<UserClaims>)> {
    let resp = self
      .client
      .post(format!("{}/api/v1/users/validate", self.addr))
      .header(http::header::AUTHORIZATION, format!("Bearer {owner_token}"))
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
    let resp = self
      .client
      .get(format!("{}/api/v1/users/profile", self.addr))
      .header(http::header::AUTHORIZATION, format!("Bearer {token}"))
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }

  #[logfn(Info)]
  pub async fn update_profile(
    &self,
    token: &str,
    req: &UpdateProfileRequest,
  ) -> anyhow::Result<(StatusCode, AppResponseResult)> {
    let resp = self
      .client
      .put(format!("{}/api/v1/users/profile", self.addr))
      .json(req)
      .header(http::header::AUTHORIZATION, format!("Bearer {token}"))
      .send()
      .await?;
    Ok((resp.status(), resp.json().await?))
  }
}
