use actix_web::web;
use actix_web_grants::PermissionGuard;
use entity::role::RoleUser;
use error::permission_denied_error;

pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.route(
    "/admin/users",
    web::get()
      .to(handler::admin::user::list)
      .guard(PermissionGuard::new(RoleUser::Admin.to_string())),
  );
  cfg.route("/admin/users", web::get().to(permission_denied_error));
}
