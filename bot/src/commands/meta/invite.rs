use std::env;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::send;

#[command]
async fn invite(ctx: &Context, msg: &Message) -> CommandResult {
    let perm = env::var("DISCORD_BOT_PERMISSION")
        .map(|bit| bit.parse::<u64>().unwrap_or(8))
        .unwrap_or(8);
    let scopes =
        env::var("DISCORD_BOT_SCOPES").unwrap_or(String::from("bot applications.commands"));
    let scopes = urlencoding::encode(&scopes);
    let client_id = env::var("DISCORD_CLIENT_ID").unwrap();
    let url = format!(
        "https://discord.com/oauth2/authorize?client_id={}&permissions={}&scope={}",
        client_id, perm, scopes
    );
    send!((ctx, msg) => "{}", url);
    Ok(())
}
