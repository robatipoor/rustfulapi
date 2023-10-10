use crate::entity::role::RoleUser;
use axum::routing::{get, post, put};

use crate::server::state::AppState;
use crate::handler::user;

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
  router
    .route("/admin/users", post(user::register))
}
