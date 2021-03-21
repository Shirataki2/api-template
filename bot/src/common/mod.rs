pub mod i18n;
pub mod redis;
pub mod slash;
pub mod tts;

pub use i18n::{get_locale, t, tt};
