use async_trait::async_trait;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::Tls;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use lettre::{AsyncTransport, Message};
use tracing::info;

use crate::configure::AppConfig;
use crate::dto::Email;
use crate::error::AppResult;

use super::ClientBuilder;

pub type EmailClient = AsyncSmtpTransport<Tokio1Executor>;

#[async_trait]
pub trait EmailClientExt: Clone + Send + Sync + ClientBuilder {
  async fn send_email(&self, email: &Email) -> AppResult;
}

impl ClientBuilder for EmailClient {
  fn build_from_config(config: &AppConfig) -> AppResult<Self> {
    Ok(
      AsyncSmtpTransport::<Tokio1Executor>::relay(&config.email.host)?
        .credentials(Credentials::new(
          config.email.username.clone(),
          config.email.password.clone(),
        ))
        .port(config.email.port)
        .tls(Tls::None)
        .build(),
    )
  }
}

#[async_trait]
impl EmailClientExt for EmailClient {
  async fn send_email(&self, email: &Email) -> AppResult {
    let resp = self.send(Message::try_from(email)?).await?;
    info!("sent email successfully code: {:?}", resp.code());
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::constant::CONFIG;
  use fake::{Fake, Faker};

  use super::*;

  #[tokio::test]
  async fn test_smtp_email_connection() {
    let client = EmailClient::build_from_config(&CONFIG).unwrap();
    assert!(client.test_connection().await.unwrap());
  }

  #[tokio::test]
  async fn test_smtp_send_email() {
    let email: Email = Faker.fake();
    let email_client = EmailClient::build_from_config(&CONFIG).unwrap();
    email_client.send_email(&email).await.unwrap();
  }
}
