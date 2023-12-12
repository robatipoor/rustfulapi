use axum::routing::get;

use crate::handler::admin;
use crate::server::state::AppState;

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
  router.route("/v1/api/admin/user/list", get(admin::user::list))
}
