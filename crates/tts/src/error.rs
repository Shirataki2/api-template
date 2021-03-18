use thiserror::Error;

#[derive(Debug, Error)]
pub enum TtsError {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("STDOUT: {0}\nSTDERR: {1}\nEXIT: {2:?}")]
    CommandError(String, String, Option<i32>),
    #[error("{0}")]
    PersistError(#[from] tempfile::PersistError),
    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("{0}")]
    DecodeError(#[from] base64::DecodeError),
}
