use fluent_templates::{Loader, static_loader};
use unic_langid::{LanguageIdentifier, langid};

const JA_JP: LanguageIdentifier = langid!("ja-JP");
const EN_US: LanguageIdentifier = langid!("en-US");

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "ja-JP",
    };
}

pub fn t(lookup_id: &str, lang: &str) -> String {
    let lang = match lang {
        "ja-JP" => JA_JP,
        "en-US" => EN_US,
        _fallback => JA_JP
    };
    info!("{:?}", lang);
    LOCALES.lookup(&lang, lookup_id)
}
