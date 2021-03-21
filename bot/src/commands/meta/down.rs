use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::{
    common::{get_locale, tt},
    data::{DatabasePool, ShardManagerContainer},
    reply,
};

#[command]
#[owners_only]
async fn down(ctx: &Context, msg: &Message) -> CommandResult {
    let pool = {
        let data = ctx.data.as_ref().read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };
    let locale = get_locale(&pool, msg).await;
    let data = ctx.data.read().await;
    if let Some(manager) = data.get::<ShardManagerContainer>() {
        reply!((ctx, msg) => "{}", tt(&locale, "ShutdownSuccess"));
        manager.lock().await.shutdown_all().await;
    } else {
        reply!((ctx, msg) => "{}", tt(&locale, "ShutdownFail"));

        return Ok(());
    }
    Ok(())
}
