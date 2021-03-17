use serenity::{client::Context, framework::standard::{macros::command, CommandResult}, model::channel::Message};

use crate::{data::ShardManagerContainer, reply};

#[command]
#[owners_only]
async fn down(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        reply!((ctx, msg) => "**Shutting Down!**");
        manager.lock().await.shutdown_all().await;
    } else {
        reply!((ctx, msg) => "シャットダウンに失敗しました");

        return Ok(());
    }
    Ok(())
}
