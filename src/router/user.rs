use axum::routing::{get, post, put};

use crate::handler::user;
use crate::server::state::AppState;

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
  router
    .route("/api/v1/users/register", post(user::register))
    .route("/api/v1/users/active", put(user::active))
    .route("/api/v1/users/login", post(user::login))
    .route("/api/v1/users/token", get(user::refresh_token))
    .route("/api/v1/users/logout", get(user::logout))
    .route("/api/v1/users/password", get(user::forget_password))
    .route("/api/v1/users/password", put(user::reset_password))
    .route("/api/v1/users/profile", get(user::get_profile))
    .route("/api/v1/users/profile", put(user::update_profile))
  // .route("/users/validate", post(permission_denied_error))
}
