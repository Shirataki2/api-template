use serenity::{
    client::{bridge::gateway::ShardId, Context},
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::{
    common::{get_locale, tt},
    data::{DatabasePool, ShardManagerContainer},
    reply,
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            reply!((ctx, msg) => "Unexpected Error");
            return Ok(());
        }
    };
    let pool = {
        let data = ctx.data.as_ref().read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };
    let locale = get_locale(&pool, msg).await;
    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = match runners.get(&ShardId(ctx.shard_id.into())) {
        Some(runner) => runner,
        None => {
            reply!((ctx, msg) => "{}", tt(&locale, "PingError"));
            return Ok(());
        }
    };
    let latency = match runner.latency {
        Some(latency) => latency,
        None => {
            reply!((ctx, msg) => "{}", tt(&locale, "PingRetry"));
            return Ok(());
        }
    };
    reply!((ctx, msg) => "🏓 Pong! **{}** ms", latency.as_millis());
    Ok(())
}
