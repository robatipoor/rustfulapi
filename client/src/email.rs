use async_trait::async_trait;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::Tls;
use lettre::{AsyncSmtpTransport, Tokio1Executor};
use lettre::{AsyncTransport, Message};
use tracing::info;

use configure::email::EmailConfig;
use error::AppResult;
use model::Email;

pub type EmailClient = AsyncSmtpTransport<Tokio1Executor>;

#[async_trait]
pub trait EmailClientExt: Clone + Send + Sync {
  async fn new(config: &EmailConfig) -> AppResult<Self>;
  async fn send_email(&self, email: &Email) -> AppResult;
}

#[async_trait]
impl EmailClientExt for EmailClient {
  async fn new(config: &EmailConfig) -> AppResult<Self> {
    Ok(
      AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)?
        .credentials(Credentials::new(
          config.username.clone(),
          config.password.clone(),
        ))
        .port(config.port)
        .tls(Tls::None)
        .build(),
    )
  }
  async fn send_email(&self, email: &Email) -> AppResult {
    let resp = self.send(Message::try_from(email)?).await?;
    info!("sent email successfully code: {:?}", resp.code());
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use configure::CONFIG;
  use fake::{Fake, Faker};

  use super::*;

  #[tokio::test]
  async fn test_smtp_email_connection() {
    let client = EmailClient::new(&CONFIG.email).await.unwrap();
    assert!(client.test_connection().await.unwrap());
  }

  #[tokio::test]
  async fn test_smtp_send_email() {
    let email: Email = Faker.fake();
    let email_client = EmailClient::new(&CONFIG.email).await.unwrap();
    email_client.send_email(&email).await.unwrap();
  }
}
