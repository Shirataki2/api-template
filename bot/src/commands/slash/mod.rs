pub mod lang;
pub mod operations;

use self::operations::Operation;
use anyhow::{bail, Context as _, Result};
use serde_json::json;
use serenity::model::interactions::Interaction;

pub async fn handle_slash_command(
    ctx: &serenity::client::Context,
    interaction: Interaction,
) -> Result<()> {
    if let Some(data) = interaction.data {
        let text_channel = interaction
            .channel_id
            .to_channel(ctx)
            .await?
            .guild()
            .context("Interaction Guild Channel")?;

        eprintln!("{:?}", data);

        let op = match data.name.as_str() {
            "lang" => Operation::Lang,
            _ => bail!("unknown command"),
        };

        let response = op
            .apply(ctx, &data, interaction.member.user.id, &text_channel)
            .await?;

        let out_json = if let Some(response) = response {
            json!({
                "type": 4,
                "data": {
                    "tts": false,
                    "content": response.message,
                    "embeds": response.embeds,
                    "allowed_mentions": [],
                    "flags": if response.ephemeral { 64 } else { 0 }
                }
            })
        } else {
            json!({
                "type": 5
            })
        };

        ctx.http
            .create_interaction_response(interaction.id.0, &interaction.token, &out_json)
            .await
            .context("Slash command response")?;
    }
    Ok(())
}
