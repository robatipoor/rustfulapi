use actix_web::web;

pub mod user;

pub fn configure(cfg: &mut web::ServiceConfig) {
  user::configure(cfg);
}
