pub mod me;

use actix_web::web;

pub fn set_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(me::me);
}