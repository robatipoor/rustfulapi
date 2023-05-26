use client::postgres::migrate_database;
use configure::{self, sentry, AppConfig};
use error::AppResult;
use server::Server;
use tracing::info;
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
  info!("init tracing success");
  let config = AppConfig::read()?;
  info!("read config file success");
  let _sentry_guard = sentry::init_sentry(&config.sentry);
  info!("init sentry done");
  let server = Server::new(config.clone()).await?;
  info!("create server");
  migrate_database(&server.state.postgres).await?;
  let state = server.state.clone();
  let server = server.run().await?;
  info!("run server");
  let server_task = tokio::task::spawn(server);
  info!("spawn server task");
  let _worker_task = server::worker::spawn(state);
  info!("spawn worker task");
  let _ = server_task.await;
  Ok(())
}
