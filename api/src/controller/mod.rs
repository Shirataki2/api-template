pub mod oauth2;
pub mod user;

use actix_web::web;

pub fn set_routes(mut cfg: &mut web::ServiceConfig) {
    oauth2::set_routes(&mut cfg);
    user::set_routes(&mut cfg);
}
