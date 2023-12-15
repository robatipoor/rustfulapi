use tracing::info;

use crate::{
  client::email::EmailClientExt,
  constant::{APP_EMAIL_ADDR, TEMPLATE_ENGIN},
  continue_if_fail,
  dto::{Email, Template},
  entity::{self, message::MessageStatus},
  error::AppResult,
  repo,
};

use super::state::AppState;

pub struct MessengerTask {
  state: AppState,
}

impl MessengerTask {
  pub fn new(state: AppState) -> Self {
    Self { state }
  }

  pub async fn run(self) -> AppResult {
    info!("The messenger task has started.");
    loop {
      let messages = match repo::message::get_list(&self.state.db, 5, 10).await {
        Ok(msg) => msg,
        Err(err) => {
          tracing::error!("Fetching the list of messages failed: {err}");
          tokio::time::sleep(std::time::Duration::from_secs(10)).await;
          continue;
        }
      };
      if messages.is_empty() {
        tokio::select! {
          _ = tokio::time::sleep(std::time::Duration::from_secs(120)) => {
            tracing::info!("The messenger task has awakened.");
          },
          _ = self.state.messenger_notify.notified() => {
            tracing::error!("The messenger task has been notified.");
          },
        }
        continue;
      }
      for message in messages {
        let user = match repo::user::find_by_id(&*self.state.db, message.user_id).await {
          Ok(user) => user.unwrap(),
          Err(err) => {
            tracing::error!("Fetching the list of users failed: {err}.");
            continue;
          }
        };
        let message_content = match render_template(&message, &user) {
          Ok(m) => m,
          Err(err) => {
            tracing::error!("Rendering the message failed: {err}.");
            continue;
          }
        };
        let email = Email::new(
          APP_EMAIL_ADDR.to_string(),
          user.email,
          message.kind.to_string(),
          message_content,
        );
        let status = match self.state.email.send_email(&email).await {
          Ok(_) => MessageStatus::Success,
          Err(err) => {
            tracing::error!("Sending the email failed: {err}.");
            MessageStatus::Failed
          }
        };
        continue_if_fail!(repo::message::update_status(&self.state.db, message, status).await);
      }
    }
  }
}

pub fn render_template(
  message: &entity::message::Model,
  user: &entity::user::Model,
) -> AppResult<String> {
  let template = match message.kind {
    entity::message::MessageKind::ActiveCode => Template::ActiveUser {
      username: user.username.clone(),
      user_id: user.id,
      code: message.content.clone(),
    },
    entity::message::MessageKind::LoginCode => Template::Login2fa {
      username: user.username.clone(),
      user_id: user.id,
      code: message.content.clone(),
    },
    entity::message::MessageKind::ForgetPasswordCode => Template::ForgetPassword {
      username: user.username.clone(),
      user_id: user.id,
      code: message.content.clone(),
    },
  };
  Ok(TEMPLATE_ENGIN.render(&template)?)
}

#[cfg(test)]
mod tests {
  use fake::Fake;

  use super::render_template;
  use crate::entity::{self, message::MessageKind};

  #[test]
  fn test_render_template() {
    let mut message: entity::message::Model = fake::Faker.fake();
    message.kind = MessageKind::LoginCode;
    let user: entity::user::Model = fake::Faker.fake();
    let result = render_template(&message, &user).unwrap();
    assert!(result.to_lowercase().contains("login"))
  }
}
