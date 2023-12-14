use tracing::info;

use crate::client::email::{EmailClient, EmailClientExt};
use crate::constant::{APP_EMAIL_ADDR, TEMPLATE_ENGIN};
use crate::dto::{Email, Template};
use crate::error::AppResult;

pub async fn send_email(
  client: &EmailClient,
  template: &Template,
  subject: &str,
  receiver_addr: &str,
) -> AppResult {
  info!("Send: {subject} email to addr: {receiver_addr}.");
  let email = create_email(template, subject, receiver_addr)?;
  client.send_email(&email).await?;
  info!("Sent the email successfully: {email:?}.");
  Ok(())
}

pub fn create_email(template: &Template, subject: &str, receiver_addr: &str) -> AppResult<Email> {
  info!("Create the email object: {template:?}.");
  Ok(Email::new(
    APP_EMAIL_ADDR.to_string(),
    receiver_addr.to_string(),
    subject.to_string(),
    TEMPLATE_ENGIN.render(template)?,
  ))
}
