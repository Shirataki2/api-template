use serenity::{client::{Context, bridge::gateway::ShardId}, framework::standard::{macros::command, CommandResult}, model::channel::Message};

use crate::{data::ShardManagerContainer, reply};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            reply!((ctx, msg) => "Shard Managerの取得に失敗しました");
            return Ok(());
        }
    };
    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = match runners.get(&ShardId(ctx.shard_id.into())) {
        Some(runner) => runner,
        None => {
            reply!((ctx, msg) => "どういうわけかShardが見当たらないようです．");
            return Ok(());
        }
    };
    let latency = match runner.latency {
        Some(latency) => latency,
        None => {
            reply!((ctx, msg) => "平均Latency計測まで時間を要するため，期間をおいて再度お試しください．");
            return Ok(());
        }
    };
    reply!((ctx, msg) => "🏓 Pong! **{}** ms", latency.as_millis());
    Ok(())
}
