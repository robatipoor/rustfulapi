
use axum::routing::{post};

use crate::server::state::AppState;
use crate::handler::user;

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
  router
    .route("/admin/users", post(user::register))
}
