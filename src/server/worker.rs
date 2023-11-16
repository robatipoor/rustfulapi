use sea_orm::TransactionTrait;
use tracing::info;

use crate::{
  client::email::EmailClientExt, constant::APP_EMAIL_ADDR, continue_if_fail, dto::Email,
  entity::message::MessageStatus, error::AppResult, repo,
};

use super::state::AppState;

pub struct MessangerTask {
  state: AppState,
}

impl MessangerTask {
  pub fn new(state: AppState) -> Self {
    Self { state }
  }

  pub async fn run(self) -> AppResult {
    info!("Messanger task start.");
    loop {
      let messages = match repo::message::get_list(&*self.state.db, 100).await {
        Ok(msg) => msg,
        Err(err) => {
          tracing::error!("{err}");
          tokio::time::sleep(std::time::Duration::from_secs(10)).await;
          continue;
        }
      };
      if messages.is_empty() {
        self.state.messanger_notify.notified().await;
        continue;
      }
      for message in messages {
        let user = match repo::user::find_by_id(&*self.state.db, message.user_id).await {
          Ok(user) => user.unwrap(),
          Err(err) => {
            tracing::error!("{err}");
            continue;
          }
        };
        let email = Email::new(
          APP_EMAIL_ADDR.to_string(),
          user.email,
          message.kind.to_string(),
          message.content.clone(),
        );
        let status = match self.state.email.send_email(&email).await {
          Ok(_) => MessageStatus::Success,
          Err(err) => {
            tracing::error!("{err}");
            MessageStatus::Failed
          }
        };
        continue_if_fail!(repo::message::update_status(&self.state.db, message, status).await);
      }
    }
  }
}
