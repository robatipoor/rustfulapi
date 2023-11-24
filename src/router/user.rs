use axum::routing::{get, post, put};

use crate::handler::user;
use crate::server::state::AppState;

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
  router
    .route("/users/register", post(user::register))
    // .route("/users/invitation", put(user::invitation))
    .route("/users/active", put(user::active))
    .route("/users/login", post(user::login))
    .route("/users/token", get(user::refresh_token))
    .route("/users/logout", get(user::logout))
    .route("/users/password", get(user::forget_password))
    .route("/users/password", put(user::reset_password))
    .route("/users/profile", get(user::get_profile))
    .route("/users/profile", put(user::update_profile))
  // .route("/users/validate", post(permission_denied_error))
}
