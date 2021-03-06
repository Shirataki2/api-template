#[macro_use]
extern crate log;

use bot::{commands, data, handler};

use std::{collections::HashSet, env, sync::Arc};

use dotenv::dotenv;

use redis::Client as RedisClient;

use serenity::{
    client::bridge::gateway::GatewayIntents, framework::StandardFramework, http::Http,
    prelude::Mutex, Client,
};

use songbird::{
    driver::{Config as DriverConfig, DecodeMode},
    SerenityInit, Songbird,
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let token = env::var("DISCORD_BOT_TOKEN").expect("Missing DISCORD_BOT_TOKEN");
    let http = Http::new_with_token(&token);

    let pg_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");
    let redis_url = env::var("REDIS_HOST").expect("Missing REDIS_HOST");
    let cert_path = env::var("GCP_SERVICE_ACCOUNT_CREDENTIAL_FILE")
        .expect("Missing GCP_SERVICE_ACCOUNT_CREDENTIAL_FILE");

    // ** Get Owner Info ** //

    let (owners, user_id) = http
        .get_current_application_info()
        .await
        .map(|info| {
            let mut set = HashSet::new();
            set.insert(info.owner.id);
            let id = info.id;
            (set, id)
        })
        .unwrap();

    // ** Framework Initialization ** //

    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners)
                .allow_dm(false)
                .on_mention(Some(user_id))
                .prefix("z.")
                .delimiters(vec![" ", "　"])
                .case_insensitivity(true)
        })
        .group(&commands::META_GROUP)
        .group(&commands::TTS_GROUP);

    let songbird = Songbird::serenity();
    songbird.set_config(DriverConfig::default().decode_mode(DecodeMode::Decode));

    // ** Setup Gateway Intents ** //

    let intents: GatewayIntents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS // Note: This intent is *priviledged*
        | GatewayIntents::GUILD_BANS
        | GatewayIntents::GUILD_EMOJIS
        | GatewayIntents::GUILD_INVITES
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGES;

    // ** Create Client ** //

    let mut client = Client::builder(&token)
        .event_handler(handler::ClientHandler::new())
        .framework(framework)
        .intents(intents)
        .register_songbird_with(songbird.into())
        .await
        .expect("Failed to create discord client");

    {
        let mut data = client.data.write().await;

        let pool = data::create_pool(&pg_url).await;

        let redis_client = RedisClient::open(redis_url).unwrap();

        let token = tts::backend::gcp::GcpToken::issue(cert_path).await.unwrap();

        data.insert::<data::RedisConnection>(Arc::new(Mutex::new(redis_client)));
        data.insert::<data::GcpAccessToken>(Arc::new(Mutex::new(token)));
        data.insert::<data::DatabasePool>(pool);
        data.insert::<data::ShardManagerContainer>(client.shard_manager.clone())
    }

    // ** Start Application ** //

    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to register CTRL + C handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    info!("Application Start");

    if let Err(e) = client.start().await {
        error!("Fatal error: {:?}", e);
    }
}
