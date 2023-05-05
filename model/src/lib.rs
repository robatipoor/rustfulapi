use fake::faker::internet::en::*;
use fake::faker::lorem::en::*;
use fake::Dummy;
use lettre::Message;
use serde::{Deserialize, Serialize};
use validator::Validate;

pub mod record;
pub mod request;
pub mod response;

pub use request::*;
pub use response::*;

#[derive(Debug, Deserialize, Serialize, Dummy, Validate)]
pub struct Email {
  #[dummy(faker = "SafeEmail()")]
  #[validate(email)]
  pub from: String,
  #[dummy(faker = "SafeEmail()")]
  #[validate(email)]
  pub to: String,
  #[validate(length(min = 1))]
  #[dummy(faker = "Sentence(5..10)")]
  pub subject: String,
  #[validate(length(min = 1))]
  #[dummy(faker = "Paragraph(5..10)")]
  pub body: String,
}

impl Email {
  pub fn new(from: String, to: String, subject: String, body: String) -> Self {
    Self {
      from,
      to,
      subject,
      body,
    }
  }
}

impl TryFrom<&Email> for Message {
  type Error = anyhow::Error;

  fn try_from(value: &Email) -> Result<Self, Self::Error> {
    Ok(
      Message::builder()
        .from(value.from.parse()?)
        // .reply_to(value.to.parse()?)
        .to(value.to.parse()?)
        .subject(value.subject.clone())
        .body(value.body.clone())?,
    )
  }
}
#[derive(Debug)]
pub enum Template {
  Invitation { username: String, code: String },
  Login { username: String, code: String },
  ForgetPassword { username: String, code: String },
}

impl Template {
  pub fn get(&self) -> (tera::Context, &'static str) {
    let mut ctx = tera::Context::new();
    match self {
      Self::Invitation { username, code } => {
        ctx.insert("username", username);
        ctx.insert("code", code);
        (ctx, "invitation.html")
      }
      Self::Login { username, code } => {
        ctx.insert("username", username);
        ctx.insert("code", code);
        (ctx, "login.html")
      }
      Self::ForgetPassword { username, code } => {
        ctx.insert("username", username);
        ctx.insert("code", code);
        (ctx, "password.html")
      }
    }
  }
}
