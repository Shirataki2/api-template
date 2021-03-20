use api_models::{guild::Guild, GetModel};
use fluent_templates::{static_loader, Loader};
use serenity::model::{channel::Message, id::GuildId};
use sqlx::PgPool;
use unic_langid::{langid, LanguageIdentifier};

const JA_JP: LanguageIdentifier = langid!("ja-JP");
const EN_US: LanguageIdentifier = langid!("en-US");

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "ja-JP",
    };
}

pub fn tt(lang: &str, lookup_id: &str) -> String {
    let lang = match lang {
        "j" | "ja-JP" => JA_JP,
        "e" | "en-US" => EN_US,
        _fallback => JA_JP,
    };
    info!("{:?}", lang);
    LOCALES.lookup(&lang, lookup_id)
}

pub async fn get_locale(pool: &PgPool, msg: &Message) -> String {
    if let Some(GuildId(gid)) = msg.guild_id {
        if let Ok(g) = Guild::get(pool, &(gid as i64)).await {
            let locale = g.locale;
            return locale;
        }
    }
    String::new()
}

pub async fn t(pool: &PgPool, msg: &Message, id: &str) -> String {
    if let Some(GuildId(gid)) = msg.guild_id {
        if let Ok(g) = Guild::get(pool, &(gid as i64)).await {
            let locale = g.locale;
            return tt(id, &locale);
        }
    }
    String::new()
}
