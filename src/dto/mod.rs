use fake::faker::internet::en::SafeEmail;
use fake::faker::lorem::en::{Paragraph, Sentence};
use fake::Dummy;
use garde::Validate;
use lettre::Message;
pub use request::*;
pub use response::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod request;
pub mod response;

#[derive(Debug, Deserialize, Serialize, Dummy, Validate)]
pub struct Email {
  #[dummy(faker = "SafeEmail()")]
  #[garde(email)]
  pub from: String,
  #[dummy(faker = "SafeEmail()")]
  #[garde(email)]
  pub to: String,
  #[garde(length(min = 1))]
  #[dummy(faker = "Sentence(5..10)")]
  pub subject: String,
  #[garde(length(min = 1))]
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
  ActiveUser {
    username: String,
    user_id: Uuid,
    code: String,
  },
  Login2fa {
    username: String,
    user_id: Uuid,
    code: String,
  },
  ForgetPassword {
    username: String,
    user_id: Uuid,
    code: String,
  },
}

impl Template {
  pub fn get(&self) -> (tera::Context, &'static str) {
    let mut ctx = tera::Context::new();
    match self {
      Self::ActiveUser {
        username,
        code,
        user_id,
      } => {
        ctx.insert("username", username);
        ctx.insert("code", code);
        ctx.insert("user_id", user_id);
        (ctx, "activation.html")
      }
      Self::Login2fa {
        username,
        code,
        user_id,
      } => {
        ctx.insert("username", username);
        ctx.insert("code", code);
        ctx.insert("user_id", user_id);
        (ctx, "login2fa.html")
      }
      Self::ForgetPassword {
        username,
        code,
        user_id,
      } => {
        ctx.insert("username", username);
        ctx.insert("code", code);
        ctx.insert("user_id", user_id);
        (ctx, "forget_password.html")
      }
    }
  }
}
