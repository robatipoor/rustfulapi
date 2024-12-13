use rustfulapi::configure;
use std::sync::LazyLock;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub mod api;
pub mod assert;
pub mod email;
pub mod http;
pub mod result;
pub mod user;

pub(crate) static INIT_SUBSCRIBER: LazyLock<()> = LazyLock::new(|| {
  configure::tracing::init_subscriber(
    Registry::default()
      .with(EnvFilter::new("INFO"))
      .with(JsonStorageLayer)
      .with(BunyanFormattingLayer::new(
        "test-app".into(),
        std::io::stdout,
      )),
  )
  .unwrap();
});
