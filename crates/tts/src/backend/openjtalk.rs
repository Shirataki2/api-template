use crate::{error::TtsError, TtsEngine};
use async_trait::async_trait;
use core::str;
use std::path::{Path, PathBuf};
use tokio::{
    fs,
    io::{AsyncWriteExt, BufWriter},
    process::Command,
};

pub struct OpenJTalk {
    pub config: OpenJTalkConfig,
}

#[async_trait]

impl TtsEngine for OpenJTalk {
    type Config = OpenJTalkConfig;

    fn from_config(config: Self::Config) -> Result<Self, TtsError> {
        Ok(Self { config })
    }

    async fn save(&self, text: &str) -> Result<String, TtsError> {
        let filename: String = uuid::Uuid::new_v4().to_string();

        let input_path = format!("{}.txt", filename);
        let output_path = format!("{}.wav", filename);
        let mut input_file = BufWriter::new(fs::File::create(&input_path).await?);
        input_file.write_all(text.as_bytes()).await?;
        input_file.flush().await?;
        self.config.execute(&input_path, &output_path).await?;
        fs::remove_file(&input_path).await?;
        Ok(output_path)
    }
}

#[derive(Clone, Debug)]
pub struct OpenJTalkConfig {
    pub dictionary: PathBuf,
    pub hts_path: PathBuf,
    pub sampling: Option<i64>,
    pub frame_period: Option<i64>,
    pub all_pass: Option<f64>,
    pub postfilter_coef: f64,
    pub speed_rate: f64,
    pub additional_half_tone: f64,
    pub unvoiced_threshold: f64,
    pub spectrum_weight: f64,
    pub spectrum_f0: f64,
}

impl Default for OpenJTalkConfig {
    fn default() -> OpenJTalkConfig {
        OpenJTalkConfig {
            dictionary: PathBuf::new(),
            hts_path: PathBuf::new(),
            sampling: None,
            frame_period: None,
            all_pass: None,
            postfilter_coef: 0.0,
            speed_rate: 1.0,
            additional_half_tone: 0.0,
            unvoiced_threshold: 0.5,
            spectrum_weight: 1.0,
            spectrum_f0: 1.0,
        }
    }
}

impl OpenJTalkConfig {
    pub async fn execute<P: AsRef<Path>>(
        &self,
        input_path: P,
        output_path: P,
    ) -> Result<(), TtsError> {
        let output = Command::new("open_jtalk")
            .arg("-x")
            .arg(&self.dictionary)
            .arg("-m")
            .arg(&self.hts_path)
            .arg("-a")
            .arg(format!("{}", self.all_pass.unwrap_or_default()))
            .arg("-b")
            .arg(format!("{}", self.postfilter_coef))
            .arg("-r")
            .arg(format!("{}", self.speed_rate))
            .arg("-fm")
            .arg(format!("{}", self.additional_half_tone))
            .arg("-u")
            .arg(format!("{}", self.unvoiced_threshold))
            .arg("-jm")
            .arg(format!("{}", self.spectrum_weight))
            .arg("-jf")
            .arg(format!("{}", self.spectrum_f0))
            .arg("-ow")
            .arg(output_path.as_ref())
            .arg(input_path.as_ref())
            .output()
            .await?;

        if output.status.success() {
            Ok(())
        } else {
            Err(TtsError::CommandError(
                String::from_utf8_lossy(&output.stdout).into(),
                String::from_utf8_lossy(&output.stderr).into(),
                output.status.code(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[tokio::test]

    async fn test_openjtalk() {
        let dictionary = PathBuf::from_str("/usr/local/dic/").unwrap();
        let hts_path =
            PathBuf::from_str("/root/.config/resources/voice/mei_normal.htsvoice").unwrap();
        let config = OpenJTalkConfig {
            dictionary,
            hts_path,
            sampling: Some(24000),
            all_pass: Some(0.54),
            postfilter_coef: 0.8,
            ..Default::default()
        };
        let engine = OpenJTalk::from_config(config).unwrap();
        let file = engine
            .save("効率的で信頼できるソフトウェアを誰もがつくれる言語")
            .await
            .unwrap();
        println!("{:?}", file);
    }
}
