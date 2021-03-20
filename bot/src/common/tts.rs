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
            let voice = GcpVoiceRequest {
                language_code: LanguageCode::JaJP.to_string(),
                name: VoiceCode::JaJPWavenetA.to_string(),
                ssml_gender: GenderCode::Female.to_string(),
            };

            let audio_config = GcpAudioConfigRequest {
                audio_encoding: "MP3".to_string(),
                ..Default::default()
            };

            let config = GcpConfig::new(&token, voice, audio_config);

            let engine = GcpTts::from_config(config)?;
            Ok(TtsKind::Gcp(engine))
        }
    }
}
