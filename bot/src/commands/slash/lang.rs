use super::operations::Response;
use crate::{common::slash::extract_option, data::DatabasePool};
use anyhow::Context as _;
use api_models::guild::Guild;
use serenity::model::{
    channel::GuildChannel, id::UserId, interactions::ApplicationCommandInteractionData,
};

pub async fn lang(
    ctx: &serenity::client::Context,
    data: &ApplicationCommandInteractionData,
    _user_id: UserId,
    text_channel: &GuildChannel,
) -> anyhow::Result<Option<Response>> {
    let pool = {
        let data = ctx.data.as_ref().read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };

    let guild_id = text_channel.guild_id.0 as i64;

    let lang = extract_option(data, "lang").context("No option")?;

    Guild::set_locale(&pool, &guild_id, &lang).await?;

    let resp = Response {
        message: format!("Set Language to: {}", lang),
        embeds: vec![],
        ephemeral: false,
    };
    Ok(Some(resp))
}
