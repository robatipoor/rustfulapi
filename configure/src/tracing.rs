use tracing::{subscriber, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

pub fn create_subscriber<W>(
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
