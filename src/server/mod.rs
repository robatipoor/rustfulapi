use self::state::AppState;
use crate::configure::AppConfig;
use crate::error::AppResult;
use crate::router::create_router_app;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
pub mod state;
pub mod worker;

pub struct AppServer {
  pub state: AppState,
  tcp: tokio::net::TcpListener,
}
impl AppServer {
  pub async fn new(mut config: AppConfig) -> AppResult<Self> {
    let tcp = tokio::net::TcpListener::bind(config.server.get_socket_addr()?).await?;
    let addr = tcp.local_addr()?;
    tracing::info!("The server is listening on: {addr}");
    config.server.port = addr.port();
    let state = AppState::new(config).await?;
    Ok(Self { state, tcp })
  }

  pub async fn run(self) -> AppResult<()> {
    let shutdown = self.grace_shutdown_time();
    let router = create_router_app(self.state)
      .layer(TraceLayer::new_for_http()) // Visibility of the request and response, change as needed.
      .layer(TimeoutLayer::new(shutdown)); // Graceful shutdown for hosting services requries N time to complete the request before shutting down.

    axum::serve(self.tcp, router)
      .with_graceful_shutdown(shutdown_signal())
      .await?;
    Ok(())
  }

  fn grace_shutdown_time(&self) -> std::time::Duration {
    std::time::Duration::from_secs(self.state.config.server.grace_shutdown_secs as u64)
  }
}

async fn shutdown_signal() {
  let ctrl_c = async {
    tokio::signal::ctrl_c()
      .await
      .expect("Failed to install CTRL+C signal handler");
  };

  #[cfg(unix)]
  let terminate = async {
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
      .expect("Failed to install SIGTERM signal handler")
      .recv()
      .await;
  };

  #[cfg(not(unix))]
  let terminate = std::future::pending::<()>();

  tokio::select! {
      _ = ctrl_c => {
          info!("Received Ctrl-C signal. Shutting down...");
      },
      _ = terminate => {
          info!("Received SIGTERM signal. Shutting down...");
      },
  }
}
