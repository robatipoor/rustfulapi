use tracing::{subscriber, Subscriber};
use tracing_appender::{
  non_blocking::WorkerGuard,
  rolling::{RollingFileAppender, Rotation},
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

use crate::error::AppResult;

fn create_subscriber<W>(
  name: &str,
  env_filter: EnvFilter,
  writer: W,
) -> impl Subscriber + Sync + Send
where
  W: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
  Registry::default()
    .with(env_filter)
    .with(JsonStorageLayer)
    .with(BunyanFormattingLayer::new(name.into(), std::io::stdout))
    .with(BunyanFormattingLayer::new(name.into(), writer))
}

pub fn init_subscriber<S>(subscriber: S) -> anyhow::Result<()>
where
  S: Subscriber + Send + Sync + 'static,
{
  LogTracer::init()?;
  subscriber::set_global_default(subscriber)?;
  Ok(())
}

pub fn init() -> AppResult<WorkerGuard> {
  let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
  let (file_appender, file_appender_guard) = tracing_appender::non_blocking(file_appender);
  init_subscriber(create_subscriber(
    "app",
    EnvFilter::from_default_env(),
    file_appender,
  ))?;
  Ok(file_appender_guard)
}
