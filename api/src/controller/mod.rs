pub mod oauth2;
pub mod user;

use std::time::Duration;

use actix_ratelimit::{RateLimiter, RedisStore, RedisStoreActor};
use actix_web::{web, HttpResponse, Responder, ResponseError};

use crate::error::AppError;

fn scope(path: &str) -> actix_web::Scope {
    web::scope(path).default_service(web::route().to(|| AppError::NotFound.error_response()))
}

fn resource(path: &str) -> actix_web::Resource {
    web::resource(path)
        .default_service(web::route().to(|| AppError::MethodNotAllowed.error_response()))
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("it works!")
}

pub fn set_routes(cfg: &mut web::ServiceConfig, redis_addr: &str) {
    let store = RedisStore::connect(redis_addr);
    cfg.service(
        scope("/")
            .app_data(web::PathConfig::default().error_handler(|_, _| AppError::NotFound.into()))
            .service(resource("").route(web::get().to(index)))
            .service(
                scope("auth")
                    .service(resource("redirect").route(web::get().to(oauth2::auth::auth)))
                    .service(resource("login").route(web::get().to(oauth2::login::login)))
                    .service(resource("logout").route(web::get().to(oauth2::logout::logout))),
            )
            .service(
                scope("users").service(
                    resource("@me").route(web::get().to(user::me::me)).wrap(
                        RateLimiter::new(RedisStoreActor::from(store).start())
                            .with_interval(Duration::from_secs(60))
                            .with_max_requests(100),
                    ),
                ),
            ),
    );
}
