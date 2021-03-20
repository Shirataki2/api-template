use redis::{AsyncCommands, FromRedisValue, RedisError, ToRedisArgs};
use serenity::client::Context;

use crate::data::RedisConnection;

pub async fn set<'a, K, V>(ctx: &Context, key: K, value: V) -> Result<(), RedisError>
where
    K: ToRedisArgs + Send + Sync + 'a,
    V: ToRedisArgs + Send + Sync + 'a,
{
    let data = ctx.data.as_ref().read().await;
    let con = data.get::<RedisConnection>().unwrap().clone();
    let con = con.lock().await;
    let mut con = con.get_async_connection().await?;
    let _: String = con.set(key, value).await?;
    Ok(())
}

pub async fn get<'a, K, RV>(ctx: &Context, key: K) -> Result<RV, RedisError>
where
    K: ToRedisArgs + Send + Sync + 'a,
    RV: FromRedisValue,
{
    let data = ctx.data.as_ref().read().await;
    let con = data.get::<RedisConnection>().unwrap().clone();
    let con = con.lock().await;
    let mut con = con.get_async_connection().await?;
    let rv = con.get(key).await?;
    Ok(rv)
}

pub async fn del<'a, K>(ctx: &Context, key: K) -> Result<(), RedisError>
where
    K: ToRedisArgs + Send + Sync + 'a,
{
    let data = ctx.data.as_ref().read().await;
    let con = data.get::<RedisConnection>().unwrap().clone();
    let con = con.lock().await;
    let mut con = con.get_async_connection().await?;
    con.del(key).await?;
    Ok(())
}
