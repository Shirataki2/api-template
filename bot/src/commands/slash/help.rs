use super::operations::Response;
use crate::common::{t, slash::extract_option};
use serenity::model::{
    channel::GuildChannel,
    id::UserId,
    interactions::ApplicationCommandInteractionData,
};
pub async fn help(
    _ctx: &serenity::client::Context,
    data: &ApplicationCommandInteractionData,
    _user_id: UserId,
    _text_channel: &GuildChannel,
) -> anyhow::Result<Option<Response>> {
    // let embed = Embed::fake(|e| {
    //     e.title("Embed title")
    //         .description("Making a basic embed")
    //         .field("A field", "Has some content.", false)
    // });
    let lang = extract_option(&data, String::from("ja-JP"));
    let resp = Response {
        message: String::from(t("hello", &lang)),
        embeds: vec![],
        ephemeral: false,
    };
    Ok(Some(resp))
}
