use super::operations::Response;
use crate::{common::tt, data::DatabasePool};
use api::GetModel;
use api_models::guild::Guild;
use serenity::model::{
    channel::GuildChannel, id::UserId, interactions::ApplicationCommandInteractionData,
};

pub async fn set_voice(
    ctx: &serenity::client::Context,
    data: &ApplicationCommandInteractionData,
    _user_id: UserId,
    text_channel: &GuildChannel,
) -> anyhow::Result<Option<Response>> {
    let pool = {
        let data = ctx.data.as_ref().read().await;
        data.get::<DatabasePool>().unwrap().clone()
    };

    let opt = {
        let op = data.options.first().unwrap();
        op.value.clone().unwrap().to_string().replace("\"", "")
    };
    let guild_id = text_channel.guild_id.0 as i64;

    Guild::set_voice(&pool, &guild_id, &opt).await?;
    let locale = Guild::get(&pool, &guild_id).await?.locale;

    let resp = Response {
        message: format!("{} **{}**", tt(&locale, "UpdateVoiceModel"), &opt),
        embeds: vec![],
        ephemeral: false,
    };

    Ok(Some(resp))
}
