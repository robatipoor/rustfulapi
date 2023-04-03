use client::{
  email::{EmailClient, EmailClientExt},
  http::{HttpClient, HttpClientExt},
  postgres::{PgClient, PgPoolExt},
  redis::{RedisClient, RedisClientExt},
};
use configure::AppConfig;
use error::AppResult;

pub mod worker;

pub struct AppState {
  pub config: AppConfig,
  pub redis: RedisClient,
  pub postgres: PgClient,
  pub email: EmailClient,
  pub http: HttpClient,
}

impl AppState {
  pub async fn new(config: AppConfig) -> AppResult<Self> {
    let redis = RedisClient::new(&config.redis)?;
    let email = EmailClient::new(&config.email).await?;
    let postgres = PgClient::new(&config.db).await?;
    let http = HttpClient::build(&config.http)?;
    Ok(Self {
      config,
      postgres,
      redis,
      email,
      http,
    })
  }
}
