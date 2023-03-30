use actix_web::web;

pub mod admin;
pub mod server;
pub mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
  server::configure(cfg);
  user::configure(cfg);
  admin::configure(cfg);
}
