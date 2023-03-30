use std::fmt::{Debug, Display};

use client::postgres::migrate_database;
use configure::{self, sentry, AppConfig};
use error::AppResult;
use server::Server;
use tokio::task::JoinError;
use tracing::{error, info};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> AppResult<()> {
  let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
  let (file_appender, _file_appender_guard) = tracing_appender::non_blocking(file_appender);
  configure::tracing::init_subscriber(configure::tracing::create_subscriber(
    "app",
    EnvFilter::from_default_env(),
    file_appender,
  ))?;
  let config = AppConfig::read()?;
  let _sentry_guard = sentry::init_sentry(&config.sentry);
  let server = Server::new(config.clone()).await?;
  migrate_database(&server.state.postgres).await?;
  let state = server.state.clone();
  let server = server.run().await?;
  let server_task = tokio::task::spawn(server);
  let worker_task = server::worker::start(state);
  tokio::select! {
      o = server_task => report_exit("main task server", o),
      o = worker_task =>  report_exit("background task worker", o),
  };
  Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
  match outcome {
    Ok(Ok(())) => {
      info!("{task_name} has exited")
    }
    Ok(Err(e)) => {
      error!(
          error.cause_chain = ?e,
          error.message = %e,
          "{task_name} failed",
      )
    }
    Err(e) => {
      error!(
          error.cause_chain = ?e,
          error.message = %e,
          "{task_name} task failed to complete",
      )
    }
  }
}
