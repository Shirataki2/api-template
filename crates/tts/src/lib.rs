#[macro_use]
extern crate serde;
#[macro_use]
extern crate log;

#[cfg(all(feature = "tokio_02", not(feature = "tokio_10")))]
extern crate tokio_compat as tokio;

#[cfg(all(feature = "tokio_02", not(feature = "tokio_10")))]
extern crate reqwest_compat as reqwest;

#[cfg(all(feature = "tokio_02", not(feature = "tokio_10")))]
extern crate hyper_compat as hyper;

#[cfg(all(feature = "tokio_02", not(feature = "tokio_10")))]
extern crate hyper_rustls_compat as hyper_rustls;

#[cfg(all(feature = "tokio_02", not(feature = "tokio_10")))]
extern crate yup_oauth2_compat as yup_oauth2;

#[cfg(any(
    all(feature = "tokio_02", feature = "tokio_10"),
    all(not(feature = "tokio_02"), not(feature = "tokio_10"))
))]
compile_error!("Runtime feature should be set to either `tokio_02` or `tokio_10`");

pub mod backend;
pub mod error;

use async_trait::async_trait;

use error::TtsError;

#[async_trait]
pub trait TtsEngine: Sized {
    type Config: Send + Sync + Clone;

    fn from_config(config: Self::Config) -> Result<Self, TtsError>;

    async fn save(&self, text: &str) -> Result<String, TtsError>;
}
