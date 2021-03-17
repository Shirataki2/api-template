use serenity::{client::bridge::gateway::ShardManager, prelude::Mutex, prelude::TypeMapKey};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct DatabasePool;

impl TypeMapKey for DatabasePool {
    type Value = PgPool;
}

pub async fn create_pool(pg_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(pg_url)
        .await
        .unwrap()
}