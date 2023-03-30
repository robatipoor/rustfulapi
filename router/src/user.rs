use actix_web::web;
use actix_web_grants::PermissionGuard;

use entity::role::RoleUser;
use error::permission_denied_error;

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.route("/users/register", web::post().to(handler::user::register));
  cfg.route(
    "/users/invitation",
    web::put().to(handler::user::invitation),
  );
  cfg.route("/users/active", web::put().to(handler::user::active));
  cfg.route("/users/login", web::post().to(handler::user::login));
  cfg.route("/users/token", web::get().to(handler::user::refresh_token));
  cfg.route("/users/logout", web::get().to(handler::user::logout));
  cfg.route(
    "/users/password",
    web::get().to(handler::user::forget_password),
  );
  cfg.route(
    "/users/password",
    web::put().to(handler::user::reset_password),
  );
  cfg.route("/users/profile", web::get().to(handler::user::get_profile));
  cfg.route(
    "/users/profile",
    web::put().to(handler::user::update_profile),
  );
  cfg.route(
    "/users/validate",
    web::post()
      .to(handler::user::validate)
      .guard(PermissionGuard::new(RoleUser::System.to_string())),
  );
  cfg.route("/users/validate", web::post().to(permission_denied_error));
}
