#[macro_use]
extern crate log;

use actix_redis::SameSite;
use actix_session::CookieSession;
use actix_web::{middleware::Logger, App, HttpServer};
use anyhow::Result;
use dotenv::dotenv;
use sqlx::PgPool;
use std::{env, ops::Deref};

use api::{
    controller::set_routes,
    data::{DiscordOauthProviderBuilder, DiscordOauthScope, HttpClient, OauthClient},
};

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    // ** LOAD CONFIG ** //

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPool::builder()
        .max_size(25)
        .build(&database_url)
        .await
        .expect("Failed to connect to database");

    let host = env::var("HOST").unwrap_or(String::from("0.0.0.0"));
    let port = env::var("PORT").unwrap_or(String::from("4040"));

    let redis_host = env::var("REDIS_HOST").expect("REDIS_HOST is not set.");
    let session_key = env::var("SESSION_KEY").expect("SESSION_KEY is not set");

    let client_id = env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID is not set");
    let client_secret =
        env::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET is not set");
    let redirect_url = env::var("DISCORD_REDIRECT_URL").expect("DISCORD_REDIRECT_URL is not set");

    // ** END OF LOAD CONFIG ** //

    use DiscordOauthScope::*;

    let provider = DiscordOauthProviderBuilder::default()
        .client_id(client_id)
        .client_secret(client_secret)
        .scopes(vec![Identify, Guilds])
        .redirect_url(redirect_url)
        .build()
        .expect("Failed to build oauth provider");

    let oauth = OauthClient::new(provider);

    let client = HttpClient::new();

    info!("Service started on http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(oauth.clone())
            .data(client.clone())
            .wrap(Logger::default())
            .wrap(
                CookieSession::signed(session_key.deref().as_bytes())
                    .same_site(SameSite::Lax)
                    .http_only(true)
                    .secure(true)
                    .domain(".chomama.jp"),
            )
            .configure(|c| set_routes(c, &redis_host))
    })
    .bind(format!("{}:{}", host, port))
    .expect("Failed to bind server")
    .run()
    .await?;

    Ok(())
}
