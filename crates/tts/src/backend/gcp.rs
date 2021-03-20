use std::{
    io::Write,
    iter,
    path::{Path, PathBuf},
};

use crate::{error::TtsError, TtsEngine};
use async_trait::async_trait;
use enum_product::enum_product;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use reqwest::Client;
use tempfile::NamedTempFile;
use yup_oauth2 as oauth;

#[derive(Clone, Debug)]
pub struct GcpToken(String, PathBuf);

impl GcpToken {
    pub async fn issue<P: AsRef<Path>>(cert_path: P) -> Result<Self, TtsError> {
        let key = oauth::read_service_account_key(cert_path.as_ref()).await?;
        let authenticator = oauth::ServiceAccountAuthenticator::builder(key)
            .build()
            .await?;
        let token = authenticator
            .token(&["https://www.googleapis.com/auth/cloud-platform"])
            .await?;
        Ok(Self(
            token.as_str().to_string(),
            cert_path.as_ref().to_path_buf(),
        ))
    }

    pub fn show(&self) -> String {
        self.0.clone()
    }

    pub async fn renew_token(&mut self) -> Result<(), TtsError> {
        let key = oauth::read_service_account_key(&self.1).await?;
        let authenticator = oauth::ServiceAccountAuthenticator::builder(key)
            .build()
            .await?;
        let token = authenticator
            .token(&["https://www.googleapis.com/auth/cloud-platform"])
            .await?;
        self.0 = token.as_str().to_string();
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct GcpTts {
    pub config: GcpConfig,
}

#[async_trait]
impl TtsEngine for GcpTts {
    type Config = GcpConfig;

    fn from_config(config: Self::Config) -> Result<Self, crate::error::TtsError> {
        Ok(Self { config })
    }

    async fn save(&self, text: &str) -> Result<String, crate::error::TtsError> {
        let client = Client::builder().gzip(true).build()?;
        let token = if self.config.access_token.trim().starts_with("Bearer ") {
            self.config.access_token.replace("Bearer ", "")
        } else {
            self.config.access_token.clone()
        };
        let url = "https://texttospeech.googleapis.com/v1/text:synthesize";

        let req = GcpRequest {
            input: GcpInputRequest {
                text: text.to_string(),
            },
            voice: self.config.voice.clone(),
            audio_config: self.config.audio_config.clone(),
        };
        info!("Request: {:?}", serde_json::to_string(&req));

        let buff = client
            .post(url)
            .bearer_auth(token)
            .json(&req)
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

        output_file.persist(format!("{}.mp3", filename))?;

        Ok(format!("{}.mp3", filename))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GcpResponse {
    pub audio_content: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GcpRequest {
    pub input: GcpInputRequest,
    pub voice: GcpVoiceRequest,
    pub audio_config: GcpAudioConfigRequest,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GcpInputRequest {
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GcpVoiceRequest {
    pub language_code: String,
    pub name: String,
    pub ssml_gender: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GcpAudioConfigRequest {
    pub audio_encoding: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub speaking_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pitch: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub voice_gain_db: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sample_rate_hertz: Option<u64>,
}

impl GcpResponse {
    pub fn decode(&self) -> Result<Vec<u8>, TtsError> {
        let buff = base64::decode(&self.audio_content)?;
        Ok(buff)
    }
}

#[derive(Clone, Debug)]
pub struct GcpConfig {
    pub access_token: String,
    pub voice: GcpVoiceRequest,
    pub audio_config: GcpAudioConfigRequest,
}

impl GcpConfig {
    pub fn new(
        token: &str,
        voice: GcpVoiceRequest,
        audio_config: GcpAudioConfigRequest,
    ) -> GcpConfig {
        Self {
            access_token: token.to_string(),
            voice,
            audio_config,
        }
    }
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
    use yup_oauth2 as oauth;

    #[tokio::test]
    async fn test_gcp() {
        dotenv::dotenv().ok();
        let cert_path = std::env::var("GCP_SERVICE_ACCOUNT_CREDENTIAL_FILE").unwrap();
        let access_token = GcpToken::issue(cert_path).await.unwrap().show();

        let voice = GcpVoiceRequest {
            language_code: LanguageCode::JaJP.to_string(),
            name: VoiceCode::JaJPWavenetA.to_string(),
            ssml_gender: GenderCode::Female.to_string(),
        };

        let audio_config = GcpAudioConfigRequest {
            audio_encoding: "MP3".to_string(),
            ..Default::default()
        };

        let config = GcpConfig::new(&access_token, voice, audio_config);

        let engine = GcpTts::from_config(config).unwrap();
        let file = engine
            .save("効率的で信頼できるソフトウェアを誰もがつくれる言語")
            .await
            .unwrap();
        println!("{:?}", file);
    }

    #[tokio::test]
    async fn test_oauth() {
        dotenv::dotenv().ok();
        let cert_path = std::env::var("GCP_SERVICE_ACCOUNT_CREDENTIAL_FILE").unwrap();
        let key = oauth::read_service_account_key(cert_path).await.unwrap();
        let authenticator = oauth::ServiceAccountAuthenticator::builder(key)
            .build()
            .await
            .unwrap();
        let token = authenticator
            .token(&["https://www.googleapis.com/auth/cloud-platform"])
            .await
            .unwrap();
        println!("{:?}", token);
    }
}
