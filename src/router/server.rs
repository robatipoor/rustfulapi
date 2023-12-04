use axum::routing::get;

use crate::{handler::server, server::state::AppState};

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
  router
    .route("/api/v1/server/health_check", get(server::health_check))
    .route("/api/v1/server/state", get(server::server_state))
}
