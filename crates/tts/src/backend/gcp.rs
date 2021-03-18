use std::{io::Write, iter};

use crate::{error::TtsError, TtsEngine};
use async_trait::async_trait;
use enum_product::enum_product;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::Client;
use tempfile::NamedTempFile;

pub struct GcpTts {
    config: GcpConfig,
}

#[async_trait]
impl TtsEngine for GcpTts {
    type Config = GcpConfig;

    fn from_config(config: Self::Config) -> Result<Self, crate::error::TtsError> {
        Ok(Self { config })
    }

    async fn save(&self, text: &str) -> Result<std::fs::File, crate::error::TtsError> {
        let client = Client::builder().gzip(true).build()?;
        let token = if self.config.access_token.trim().starts_with("Bearer ") {
            self.config.access_token.replace("Bearer ", "")
        } else {
            self.config.access_token.clone()
        };
        let url = "https://texttospeech.googleapis.com/v1/text:synthesize";
        let body = GcpRequest::new(
            text.to_string(),
            self.config.language_code.to_string(),
            self.config.name.to_string(),
            self.config.ssml_gender.to_string(),
            String::from("MP3"),
        );

        let buff = client
            .post(url)
            .bearer_auth(token)
            .json(&body)
            .send()
            .await?
            .json::<GcpResponse>()
            .await?
            .decode()?;

        let mut output_file = NamedTempFile::new()?;
        output_file.write_all(&buff)?;
        let mut rng = thread_rng();
        let filename: String = iter::repeat(())
            .map(|_| rng.sample(Alphanumeric))
            .map(char::from)
            .take(32)
            .collect();

        let file = output_file.persist(format!("{}.mp3", filename))?;

        Ok(file)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GcpResponse {
    audio_content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GcpRequest {
    input: GcpInputRequest,
    voice: GcpVoiceRequest,
    audio_config: GcpAudioConfigRequest,
}

impl GcpRequest {
    fn new(
        text: String,
        language_code: String,
        name: String,
        ssml_gender: String,
        audio_encoding: String,
    ) -> Self {
        GcpRequest {
            input: GcpInputRequest { text },
            voice: GcpVoiceRequest {
                language_code,
                name,
                ssml_gender,
            },
            audio_config: GcpAudioConfigRequest { audio_encoding },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GcpInputRequest {
    text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GcpVoiceRequest {
    language_code: String,
    name: String,
    ssml_gender: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GcpAudioConfigRequest {
    audio_encoding: String,
}

impl GcpResponse {
    pub fn decode(&self) -> Result<Vec<u8>, TtsError> {
        let buff = base64::decode(&self.audio_content)?;
        Ok(buff)
    }
}

#[derive(Clone, Debug)]
pub struct GcpConfig {
    access_token: String,
    language_code: LanguageCode,
    name: VoiceCode,
    ssml_gender: GenderCode,
}

enum_product! {
    pub enum LanguageCode {
        [
            "ar-XA", "bn-IN", "yue-HK", "cs-CZ", "da-DK",
            "nl-NL", "en-AU", "en-IN", "en-GB", "en-US",
            "fil-PH", "fi-FI", "fr-CA", "fr-FR", "de-DE",
            "el-GR", "hi-IN", "hu-HU", "id-ID", "it-IT",
            "ja-JP", "kn-IN", "ko-KR", "cmn-CN", "cmn-TW",
            "nb-NO", "pl-PL", "pt-PT", "ru-RU", "es-ES",
            "tr-TR", "vi-VN"
        ]
    }
}

enum_product! {
    pub enum VoiceCode {
        [
            "ar-XA", "bn-IN", "yue-HK", "cs-CZ", "da-DK",
            "nl-NL", "en-AU", "en-IN", "en-GB", "en-US",
            "fil-PH", "fi-FI", "fr-CA", "fr-FR", "de-DE",
            "el-GR", "hi-IN", "hu-HU", "id-ID", "it-IT",
            "ja-JP", "kn-IN", "ko-KR", "cmn-CN", "cmn-TW",
            "nb-NO", "pl-PL", "pt-PT", "ru-RU", "es-ES",
            "tr-TR", "vi-VN"
        ],
        ["-Standard-", "-Wavenet-"],
        ["A", "B", "C", "D", "E"]
    }
}

#[derive(Clone, Debug)]
pub enum GenderCode {
    Male,
    Female,
}

impl std::string::ToString for GenderCode {
    fn to_string(&self) -> String {
        use GenderCode::*;
        match self {
            &Male => "MALE",
            &Female => "FEMALE",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    #[cfg(all(feature = "tokio_10", not(feature = "tokio_02")))]
    use tokio;
    #[cfg(all(feature = "tokio_02", not(feature = "tokio_10")))]
    use tokio_compat as tokio;

    use super::*;

    #[tokio::test]
    async fn test_gcp() {
        dotenv::dotenv().ok();
        let access_token = std::env::var("GCP_ACCESS_TOKEN").unwrap();
        let config = GcpConfig {
            access_token,
            language_code: LanguageCode::JaJP,
            name: VoiceCode::JaJPWavenetA,
            ssml_gender: GenderCode::Female,
        };
        let engine = GcpTts::from_config(config).unwrap();
        let file = engine
            .save("効率的で信頼できるソフトウェアを誰もがつくれる言語")
            .await
            .unwrap();
        println!("{:?}", file);
    }
}
