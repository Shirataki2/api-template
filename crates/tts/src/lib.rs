#[macro_use]
extern crate serde;

#[cfg(all(feature = "tokio_02", not(feature = "tokio_10")))]
extern crate reqwest_compat as reqwest;

#[cfg(any(all(feature = "tokio_02", feature = "tokio_10"), all(not(feature = "tokio_02"), not(feature = "tokio_10"))))]
compile_error!("Runtime feature should be set to either `tokio_02` or `tokio_10`");

pub mod backend;
pub mod error;

use async_trait::async_trait;

use error::TtsError;
use std::fs::File;

#[async_trait]
pub trait TtsEngine: Sized {
    type Config: Send + Sync + Clone;

    fn from_config(config: Self::Config) -> Result<Self, TtsError>;

    async fn save(&self, text: &str) -> Result<File, TtsError>;
}
