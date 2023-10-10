use futures::FutureExt;
use rustfulapi::constant::CONFIG;
use rustfulapi::error::AppResult;
use rustfulapi::server::AppServer;
use rustfulapi::{configure, util};
use tracing::info;

#[tokio::main]
async fn main() -> AppResult<()> {
  let _file_appender_guard = configure::tracing::init()?;
  info!("init tracing success");
  let config = CONFIG.clone();
  info!("read config file success");
  let _sentry_guard = configure::sentry::init(&config.sentry);
  info!("init sentry done");
  info!("create server");
  let server = AppServer::new(config).await?;
  info!("run server");
  util::task::join_all(vec![(true, server.run().boxed())]).await?;
  Ok(())
}
