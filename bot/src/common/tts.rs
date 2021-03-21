use std::{path::PathBuf, str::FromStr};

use tts::{
    backend::{
        gcp::{
            GcpAudioConfigRequest, GcpConfig, GcpTts, GcpVoiceRequest, GenderCode, LanguageCode,
            VoiceCode,
        },
        openjtalk::{OpenJTalk, OpenJTalkConfig},
    },
    error::TtsError,
    TtsEngine,
};

pub enum TtsType {
    OpenJTalkMeiNormal,
    GcpJpFemaleNormalA(String),
    GcpJpFemaleNormalB(String),
    GcpJpFemalePremiumA(String),
    GcpJpFemalePremiumB(String),
    GcpJpMaleNormalA(String),
    GcpJpMaleNormalB(String),
    GcpJpMalePremiumA(String),
    GcpJpMalePremiumB(String),
}

pub enum TtsKind {
    OpenJTalk(OpenJTalk),
    Gcp(GcpTts),
}

impl TtsKind {
    pub fn ensure_ojtalk(self) -> OpenJTalk {
        match self {
            TtsKind::OpenJTalk(e) => e,
            _ => unreachable!(),
        }
    }

    pub fn ensure_gcp(self) -> GcpTts {
        match self {
            TtsKind::Gcp(e) => e,
            _ => unreachable!(),
        }
    }
}

macro_rules! gcp_voice {
    ($token:expr => {lang: $lang:expr, name: $name:expr, gender: $gender:expr}) => {{
        let voice = GcpVoiceRequest {
            language_code: $lang.to_string(),
            name: $name.to_string(),
            ssml_gender: $gender.to_string(),
        };

        let audio_config = GcpAudioConfigRequest {
            audio_encoding: "MP3".to_string(),
            ..Default::default()
        };

        let config = GcpConfig::new(&$token, voice, audio_config);

        let engine = GcpTts::from_config(config)?;
        Ok(TtsKind::Gcp(engine))
    }};
}

pub fn create_tts_engine(tts_type: TtsType) -> Result<TtsKind, TtsError> {
    match tts_type {
        TtsType::OpenJTalkMeiNormal => {
            let dictionary = PathBuf::from_str("/usr/local/dic/").unwrap();
            let hts_path =
                PathBuf::from_str("/root/.config/resources/voice/mei_normal.htsvoice").unwrap();

            let config = OpenJTalkConfig {
                dictionary,
                hts_path,
                all_pass: Some(0.54),
                postfilter_coef: 0.8,
                ..Default::default()
            };
            let engine = OpenJTalk::from_config(config)?;
            Ok(TtsKind::OpenJTalk(engine))
        }
        TtsType::GcpJpFemaleNormalA(token) => {
            gcp_voice!(token => {
                lang: LanguageCode::JaJP,
                name: VoiceCode::JaJPStandardA,
                gender: GenderCode::Female
            })
        }
        TtsType::GcpJpFemaleNormalB(token) => {
            gcp_voice!(token => {
                lang: LanguageCode::JaJP,
                name: VoiceCode::JaJPStandardB,
                gender: GenderCode::Female
            })
        }
        TtsType::GcpJpFemalePremiumA(token) => {
            gcp_voice!(token => {
                lang: LanguageCode::JaJP,
                name: VoiceCode::JaJPWavenetA,
                gender: GenderCode::Female
            })
        }
        TtsType::GcpJpFemalePremiumB(token) => {
            gcp_voice!(token => {
                lang: LanguageCode::JaJP,
                name: VoiceCode::JaJPWavenetB,
                gender: GenderCode::Female
            })
        }
        TtsType::GcpJpMaleNormalA(token) => {
            gcp_voice!(token => {
                lang: LanguageCode::JaJP,
                name: VoiceCode::JaJPStandardC,
                gender: GenderCode::Male
            })
        }
        TtsType::GcpJpMaleNormalB(token) => {
            gcp_voice!(token => {
                lang: LanguageCode::JaJP,
                name: VoiceCode::JaJPStandardD,
                gender: GenderCode::Male
            })
        }
        TtsType::GcpJpMalePremiumA(token) => {
            gcp_voice!(token => {
                lang: LanguageCode::JaJP,
                name: VoiceCode::JaJPWavenetC,
                gender: GenderCode::Male
            })
        }
        TtsType::GcpJpMalePremiumB(token) => {
            gcp_voice!(token => {
                lang: LanguageCode::JaJP,
                name: VoiceCode::JaJPWavenetD,
                gender: GenderCode::Male
            })
        }
    }
}
