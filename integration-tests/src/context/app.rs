use actix_web::web;
use client::postgres::*;
use configure::AppConfig;
use once_cell::sync::Lazy;
use server;
use sqlx::PgConnection;
use state::AppState;
use test_context::AsyncTestContext;
use tokio::task::JoinHandle;
use tracing::info;
use wiremock::MockServer;

use crate::helper::{api::Api, email::MailHogClient, INIT_SUBSCRIBER};

pub struct AppTestContext {
  pub tasks: Vec<JoinHandle<std::io::Result<()>>>,
  pub state: web::Data<AppState>,
  pub pg_conn: PgConnection,
  pub mock_server: MockServer,
  pub api: Api,
  pub mail: MailHogClient,
}

#[async_trait::async_trait]
impl AsyncTestContext for AppTestContext {
  async fn setup() -> Self {
    Lazy::force(&INIT_SUBSCRIBER);
    let mut config = AppConfig::read().unwrap();
    let pg_conn = setup_new_database(&mut config.db).await.unwrap();
    let server = server::Server::new(config).await.unwrap();
    migrate_database(&server.state.postgres).await.unwrap();
    let state = server.state.clone();
    let server_task = tokio::task::spawn(server.run().await.unwrap());
    let mock_server = MockServer::start().await;
    let api = Api::new(&state.config.server);
    let mail = MailHogClient::new(&state.config.email);
    let tasks = vec![server_task];
    Self {
      tasks,
      state,
      api,
      pg_conn,
      mock_server,
      mail,
    }
  }

  async fn teardown(mut self) {
    drop_database(&self.state.config.db.database_name, &mut self.pg_conn)
      .await
      .unwrap();
    for j in self.tasks {
      j.abort();
    }
    info!("teardown done successfully");
  }
}
