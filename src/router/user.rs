use axum::routing::{get, post, put};

use crate::handler::user;
use crate::server::state::AppState;

pub fn add_routers(router: axum::Router<AppState>) -> axum::Router<AppState> {
  router
    .route("/api/v1/user/register", post(user::register))
    .route("/api/v1/user/active", put(user::active))
    .route("/api/v1/user/login", post(user::login))
    .route("/api/v1/user/login2fa", post(user::login2fa))
    .route("/api/v1/user/logout", get(user::logout))
    .route("/api/v1/user/password", get(user::forget_password))
    .route("/api/v1/user/password", put(user::reset_password))
    .route("/api/v1/user/profile", get(user::get_profile))
    .route("/api/v1/user/profile", put(user::update_profile))
}
