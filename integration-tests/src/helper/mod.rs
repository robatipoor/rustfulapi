use configure::tracing;
use once_cell::sync::Lazy;
use tracing_subscriber::EnvFilter;

pub mod api;
pub mod email;
pub mod http;
pub mod result;
pub mod user;

pub static INIT_SUBSCRIBER: Lazy<()> = Lazy::new(|| {
  tracing::init_subscriber(tracing::create_subscriber(
    "test-app",
    EnvFilter::new("INFO"),
    std::io::stdout,
  ))
  .unwrap();
});
