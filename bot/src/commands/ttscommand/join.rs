use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::{
    common::{get_locale, redis::set, tt},
    data::DatabasePool,
    reply,
};

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let pool = {
        let data = ctx.data.as_ref().read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };
    let locale = get_locale(&pool, msg).await;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|vs| vs.channel_id);
    let connect_to = match channel_id {
        Some(channel_id) => channel_id,
        None => {
            reply!((ctx, msg) => "{}", tt(&locale, "NotInVoiceChannel"));
            return Ok(());
        }
    };
    let manager = match songbird::get(ctx).await {
        Some(manager) => manager.clone(),
        None => {
            reply!((ctx, msg) => "{}", tt(&locale, "UnexpectedError"));
            return Ok(());
        }
    };
    let (_lock, success) = manager.join(guild_id, connect_to).await;
    match success {
        Ok(()) => {}
        Err(e) => {
            error!("Join Error; {:?}", e);
            reply!((ctx, msg) => "{}", tt(&locale,  "JoinFailed"));
        }
    }

    reply!((ctx, msg) => "{}", tt(&locale,  "Joined"));

    if let Err(e) = set(
        &ctx,
        &format!("bot:channel:joined:{}", guild_id.0),
        msg.channel_id.0,
    )
    .await
    {
        error!("Redis Error: {:?}", e);
    }

    Ok(())
}
