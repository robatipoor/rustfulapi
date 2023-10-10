use self::state::AppState;
use crate::configure::AppConfig;
use crate::error::AppResult;
use crate::router::create_router_app;
pub mod state;

pub struct AppServer {
  pub state: AppState,
  tcp: tokio::net::TcpListener,
}
impl AppServer {
  pub async fn new(mut config: AppConfig) -> AppResult<Self> {
    let tcp = tokio::net::TcpListener::bind(config.server.get_socket_addr()?).await?;
    config.server.port = tcp.local_addr()?.port();
    let state = AppState::new(config).await?;
    Ok(Self { state, tcp })
  }

  pub async fn run(self) -> AppResult<()> {
    let app = create_router_app(self.state);
    axum::serve(self.tcp, app).await.map_err(|e| e.into())
  }
}
