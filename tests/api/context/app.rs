use once_cell::sync::Lazy;
use rustfulapi::{
  client::database::{drop_database, setup_new_database},
  configure::AppConfig,
  error::AppResult,
  server::{self, state::AppState},
};
use test_context::AsyncTestContext;
use tokio::task::JoinHandle;
use tracing::info;
use wiremock::MockServer;

use crate::helper::{api::Api, email::MailHogClient, INIT_SUBSCRIBER};

pub struct AppTestContext {
  pub tasks: Vec<JoinHandle<AppResult>>,
  pub state: AppState,
  pub mock_server: MockServer,
  pub api: Api,
  pub mail: MailHogClient,
}

#[async_trait::async_trait]
impl AsyncTestContext for AppTestContext {
  async fn setup() -> Self {
    Lazy::force(&INIT_SUBSCRIBER);
    let mut config = AppConfig::read().unwrap();
    setup_new_database(&mut config).await.unwrap();
    let server = server::AppServer::new(config).await.unwrap();
    // migrate_database(&server.state.db).await.unwrap();
    let state = server.state.clone();
    let server_task = tokio::task::spawn(server.run());
    let mock_server = MockServer::start().await;
    let api = Api::new(&state.config.server);
    let mail = MailHogClient::new(&state.config.email);
    let tasks = vec![server_task];
    Self {
      tasks,
      state,
      api,
      mock_server,
      mail,
    }
  }

  async fn teardown(mut self) {
    drop_database(&self.state.db, &self.state.config.db.database_name)
      .await
      .unwrap();
    for j in self.tasks {
      j.abort();
    }
    info!("Teardown done successfully.");
  }
}
