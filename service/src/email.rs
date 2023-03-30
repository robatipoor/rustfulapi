use constant::APP_EMAIL_ADDR;
use tracing::info;

use client::email::{EmailClient, EmailClientExt};
use configure::template::TEMPLATE_ENGIN;
use error::AppResult;
use model::{Email, Template};

pub async fn send_email(
  client: &EmailClient,
  template: &Template,
  subject: &str,
  receiver_addr: &str,
) -> AppResult {
  info!("send: {subject} email to addr: {receiver_addr}");
  let email = create_email(template, subject, receiver_addr)?;
  client.send_email(&email).await?;
  info!("sent email: {email:?} successfully");
  Ok(())
}

pub fn create_email(template: &Template, subject: &str, receiver_addr: &str) -> AppResult<Email> {
  info!("create email object: {template:?}");
  Ok(Email::new(
    APP_EMAIL_ADDR.to_string(),
    receiver_addr.to_string(),
    subject.to_string(),
    TEMPLATE_ENGIN.render(template)?,
  ))
}
