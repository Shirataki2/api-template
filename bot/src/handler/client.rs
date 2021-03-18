use crate::{commands::slash, data::DatabasePool};
use api::{
    guild::{Guild as GuildModel, GuildBuilder},
    CreateModel,
};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{gateway::Ready, guild::Guild, interactions::Interaction},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, ready: Ready) {
        info!("Logged in as {}", ready.user.name);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Err(e) = slash::handle_slash_command(&ctx, interaction).await {
            error!("Failed to reply slash command: {:?}", e);
        }
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        info!("Guild {} recieved; is_new: {}", guild.name, is_new);
        let pool = {
            let data = ctx.data.read().await;
            match data.get::<DatabasePool>() {
                Some(pool) => pool.clone(),
                None => {
                    error!("Failed to get database pool");
                    return;
                }
            }
        };
        let payload = GuildBuilder::default()
            .id(guild.id.0 as i64)
            .name(guild.name.clone())
            .icon_url(
                guild
                    .icon_url()
                    .unwrap_or("".to_string())
                    .replace(".webp", ".png"),
            )
            .locale("ja-JP".to_string())
            .build();

        let payload = match payload {
            Ok(payload) => payload,
            Err(e) => {
                error!("Failed to build payload; {:#?}", e);
                return;
            }
        };

        if let Err(e) = GuildModel::checked_create(&pool, &(guild.id.0 as i64), payload).await {
            error!("Failed to insert guild; {:#?}", e);
        }
    }
}
