pub mod login;
pub mod auth;
pub mod logout;

use actix_web::web;

pub fn set_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login::login);
    cfg.service(logout::logout);
    cfg.service(auth::auth);
}