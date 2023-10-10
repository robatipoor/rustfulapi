use crate::server::state::AppState;
use axum::Router;

pub mod admin;
pub mod server;
pub mod user;

pub fn create_router_app(state: AppState) -> Router {
  let router = Router::new();
  let router = server::add_routers(router);
  let router = user::add_routers(router);
  let router = admin::user::add_routers(router);
  router.with_state(state)
}
