use once_cell::sync::Lazy;
use rustfulapi::{
  client::database::{drop_database, migrate_database, setup_new_database, DatabaseClient},
  configure::{env::get_env_source, AppConfig},
  constant::ENV_PREFIX,
  error::AppResult,
  server::{self, state::AppState, worker::MessengerTask},
};
use test_context::AsyncTestContext;
use tokio::task::JoinHandle;
use tracing::info;
use wiremock::MockServer;

use crate::helper::{api::Api, email::MailHogClient, INIT_SUBSCRIBER};

pub struct AppTestContext {
  pub tasks: Vec<JoinHandle<AppResult>>,
  pub state: AppState,
  pub default_db: DatabaseClient,
  pub mock_server: MockServer,
  pub api: Api,
  pub mail: MailHogClient,
}

impl AsyncTestContext for AppTestContext {
  async fn setup() -> Self {
    Lazy::force(&INIT_SUBSCRIBER);
    let mut config = AppConfig::read(get_env_source(ENV_PREFIX)).unwrap();
    let default_db = setup_new_database(&mut config).await.unwrap();
    let server = server::AppServer::new(config).await.unwrap();
    migrate_database(&server.state.db).await.unwrap();
    let state = server.state.clone();
    let server_task = tokio::task::spawn(server.run());
    let messenger = MessengerTask::new(state.clone());
    let messenger_task = tokio::task::spawn(messenger.run());
    let mock_server = MockServer::start().await;
    let api = Api::new(&state.config.server);
    let mail = MailHogClient::new(&state.config.email);
    let tasks = vec![server_task, messenger_task];
    Self {
      tasks,
      state,
      api,
      default_db,
      mock_server,
      mail,
    }
  }

  async fn teardown(self) {
    drop_database(&self.default_db, &self.state.config.db.database_name)
      .await
      .unwrap();
    for j in self.tasks {
      j.abort();
    }
    info!("Teardown done successfully.");
  }
}
