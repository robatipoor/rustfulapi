use actix_web::web;
use error::permission_denied_error;

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.route(
    "/server/health_check",
    web::get().to(handler::server::health_check),
  );
  cfg.route(
    "/server/state",
    web::get().to(handler::server::server_state),
  );
  cfg.route("/server/state", web::get().to(permission_denied_error));
}
