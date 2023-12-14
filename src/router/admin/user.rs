use axum::routing::get;

use crate::handler::admin;
use crate::server::state::AppState;

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
  router.route("/api/v1/admin/user/list", get(admin::user::list))
}
