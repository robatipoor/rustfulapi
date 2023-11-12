use sea_orm::TransactionTrait;
use tracing::{error, info};

use crate::{error::AppResult, repo};

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
      let message = repo::message::get_page(&self.state.db.begin().await?).await?;
      //send message
    }
  }
}
