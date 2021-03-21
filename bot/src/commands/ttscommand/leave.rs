use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::{
    common::{get_locale, redis::del, tt},
    data::DatabasePool,
    reply,
};

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let pool = {
        let data = ctx.data.as_ref().read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };
    let locale = get_locale(&pool, msg).await;

    let manager = match songbird::get(ctx).await {
        Some(manager) => manager.clone(),
        None => {
            reply!((ctx, msg) => "{}", tt(&locale, "UnexpectedError"));
            return Ok(());
        }
    };

    if manager.get(guild_id).is_some() {
        if let Err(e) = manager.remove(guild_id).await {
            error!("Remove failed; {:?}", e);
            reply!((ctx, msg) => "{}", tt(&locale, "LeaveFailed"));
        }
    } else {
        reply!((ctx, msg) => "{}", tt(&locale, "NotInVoiceChannel"));
    }

    reply!((ctx, msg) => "{}", tt(&locale, "LeftVoiceChannel"));

    let _ = del(&ctx, &format!("bot:channel:joined:{}", guild_id.0));

    Ok(())
}
