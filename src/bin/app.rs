use futures::FutureExt;
use rustfulapi::constant::CONFIG;
use rustfulapi::error::AppResult;
use rustfulapi::server::worker::MessangerTask;
use rustfulapi::server::AppServer;
use rustfulapi::{configure, util};
use tracing::info;

#[tokio::main]
async fn main() -> AppResult<()> {
  let _file_appender_guard = configure::tracing::init()?;
  info!("Initialization of tracing was successful.");
  let config = CONFIG.clone();
  info!("Read the config file was successful.");
  let _sentry_guard = configure::sentry::init(&config.sentry);
  info!("Initialization of sentry was successful.");
  info!("Create a new server.");
  let server = AppServer::new(config).await?;
  info!("Create a new messanger task.");
  let messanger = MessangerTask::new(server.state.clone());
  info!("Run server.");
  util::task::join_all(vec![
    (true, server.run().boxed()),
    (true, messanger.run().boxed()),
  ])
  .await?;
  Ok(())
}
