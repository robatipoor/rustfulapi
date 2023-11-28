use std::time::Duration;

use async_trait::async_trait;
use reqwest::Response;
use serde::Serialize;

use crate::{configure::AppConfig, error::AppResult};

use super::ClientBuilder;

pub type HttpClient = reqwest::Client;

#[async_trait]
pub trait HttpClientExt: ClientBuilder {
  async fn post_request<T: Serialize + ?Sized + Send + Sync>(
    &self,
    url: &str,
    body: &T,
  ) -> Result<Response, reqwest::Error>;
  async fn put_request<T: Serialize + ?Sized + Send + Sync>(
    &self,
    url: &str,
    body: &T,
  ) -> Result<Response, reqwest::Error>;
  async fn delete_request(&self, url: &str) -> Result<Response, reqwest::Error>;
  async fn get_request(&self, url: &str) -> Result<Response, reqwest::Error>;
}

impl ClientBuilder for HttpClient {
  fn build_from_config(config: &AppConfig) -> AppResult<Self> {
    Ok(
      reqwest::Client::builder()
        .timeout(Duration::from_secs(config.http.timeout))
        .build()?,
    )
  }
}

#[async_trait::async_trait]
impl HttpClientExt for HttpClient {
  async fn post_request<T: Serialize + ?Sized + Send + Sync>(
    &self,
    url: &str,
    body: &T,
  ) -> Result<Response, reqwest::Error> {
    self.post(url).json(body).send().await
  }

  async fn put_request<T: Serialize + ?Sized + Send + Sync>(
    &self,
    url: &str,
    body: &T,
  ) -> Result<Response, reqwest::Error> {
    self.put(url).json(body).send().await
  }

  async fn delete_request(&self, url: &str) -> Result<Response, reqwest::Error> {
    self.delete(url).send().await
  }

  async fn get_request(&self, url: &str) -> Result<Response, reqwest::Error> {
    self.get(url).send().await
  }
}
