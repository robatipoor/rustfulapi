use axum::routing::get;

use crate::{handler::server, server::state::AppState};

pub fn add_routers(router: axum::Router<AppState> ) -> axum::Router<AppState> {
  router.route(
    "/server/health_check",
    get(server::health_check),
  ).route(
    "/server/state",
    get(server::server_state),
  )
}
