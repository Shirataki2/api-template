use crate::commands::slash;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{gateway::Ready, interactions::Interaction},
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
}
