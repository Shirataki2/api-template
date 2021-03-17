pub mod credential;
pub mod http;
pub mod oauth_client;

pub use credential::authorize;
pub use http::HttpClient;
pub use oauth_client::{
    DiscordOauthProvider, DiscordOauthProviderBuilder, DiscordOauthScope, OauthClient,
    OauthProvider,
};
