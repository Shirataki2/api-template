use std::{fs, sync::Arc};

use crate::{
    commands::slash,
    common::{
        redis::get,
        tts::{create_tts_engine, TtsType},
    },
    data::{DatabasePool, GcpAccessToken},
    tasks,
};
use api::{
    guild::{Guild as GuildModel, GuildBuilder},
    CreateModel,
};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Message,
        gateway::Ready,
        guild::Guild,
        id::{GuildId, UserId},
        interactions::Interaction,
    },
    prelude::Mutex,
};
use songbird::{
    input::restartable::Restartable, Event, EventContext, EventHandler as VoiceEventHandler,
    TrackEvent,
};
use tts::TtsEngine;

pub struct Handler {
    user_id: Mutex<Option<UserId>>,
    run_loop: Mutex<bool>,
}

impl Handler {
    pub fn new() -> Handler {
        let run_loop = Mutex::new(true);
        Self {
            user_id: Mutex::new(None),
            run_loop,
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        info!("Cache is ready");
        if *self.run_loop.lock().await {
            *self.run_loop.lock().await = false;
            let ctx = Arc::new(ctx);
            let ctx2 = Arc::clone(&ctx);
            let token_loop =
                tokio::spawn(async move { tasks::token_renewer::renew_token(ctx2).await });
            let _ = token_loop.await;
        }
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        *self.user_id.lock().await = Some(ready.user.id);
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

    async fn message(&self, ctx: Context, msg: Message) {
        // 発言条件
        // 1) Botの発言ではない
        // 2) [z.]から始まらない
        // 3) z.joinを送信したチャンネルである
        // 4) このBotがVCに入室している

        // 1) & 2)
        if msg.author.bot || msg.content.starts_with("z.") {
            return;
        }

        // 4)
        let is_bot_in_vc = match msg.guild(&ctx.cache).await {
            Some(guild) => {
                let user_id = self.user_id.lock().await;
                if let Some(user_id) = user_id.as_ref() {
                    let bot_user = guild.voice_states.get(&user_id);
                    bot_user.is_some()
                } else {
                    false
                }
            }
            None => false,
        };
        if !is_bot_in_vc {
            return;
        }

        // 4)
        let guild = match msg.guild(&ctx.cache).await {
            Some(guild) => guild,
            None => return,
        };
        let guild_id = guild.id;

        let channel_id = msg.channel_id.0;
        match get::<_, u64>(&ctx, &format!("bot:channel:joined:{}", guild_id.0)).await {
            Ok(v) => {
                if v != channel_id {
                    return;
                }
            }
            Err(e) => {
                error!("{:?}", e);
            }
        }

        let manager = match songbird::get(&ctx).await {
            Some(manager) => manager.clone(),
            None => return,
        };

        // * TTS Start * //
        let token = {
            let data = ctx.data.read().await;
            let token = data.get::<GcpAccessToken>().unwrap();
            let token = token.lock().await;
            token.show()
        };
        let engine = create_tts_engine(TtsType::GcpJpFemaleNormalA(token))
            .unwrap()
            .ensure_gcp();

        let path = match engine.save(&msg.content.replace("\n", "．")).await {
            Ok(path) => path,
            Err(e) => {
                error!("TTS save error: {:?}", e);
                return;
            }
        };

        // * TTS End * //

        if let Some(handler) = manager.get(guild_id) {
            let mut handler = handler.lock().await;
            let src = match Restartable::ffmpeg(path.clone(), true).await {
                Ok(src) => src,
                Err(e) => {
                    error!("TTS convert error; {:?}", e);
                    return;
                }
            };
            handler.enqueue_source(src.into());

            let _ = handler.add_global_event(Event::Track(TrackEvent::End), FileRemover { path });
        }
    }
}

struct FileRemover {
    path: String,
}

#[async_trait]
impl VoiceEventHandler for FileRemover {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let _ = fs::remove_file(&self.path);
        None
    }
}
