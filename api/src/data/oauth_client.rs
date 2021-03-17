use derive_builder::Builder;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, TokenUrl,
};
use urlencoding::encode;

#[derive(Debug, Clone)]
pub struct OauthClient {
    pub client: BasicClient,
}

impl OauthClient {
    pub fn new<Provider>(provider: Provider) -> Self
    where
        Provider: OauthProvider,
    {
        let auth_url = AuthUrl::new(provider.auth_url()).expect("Failed to parse auth URL");
        let token_url = provider
            .token_url()
            .map(|s| TokenUrl::new(s).expect("Failed to parse token URL"));
        let redirect_url =
            RedirectUrl::new(provider.redirect_url()).expect("Failed to parse redirect URL");

        let client = BasicClient::new(
            ClientId::new(provider.client_id()),
            provider.client_secret().map(|s| ClientSecret::new(s)),
            auth_url,
            token_url,
        )
        .set_redirect_url(redirect_url);
        Self { client }
    }

    pub fn generate_auth_url(&self) -> (String, CsrfToken, PkceCodeVerifier) {
        let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
        let (authorize_url, csrf_state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_code_challenge)
            .url();
        (authorize_url.to_string(), csrf_state, pkce_code_verifier)
    }
}

pub trait OauthProvider {
    fn client_id(&self) -> String;

    fn client_secret(&self) -> Option<String> {
        None
    }

    fn auth_url(&self) -> String;

    fn token_url(&self) -> Option<String> {
        None
    }

    fn redirect_url(&self) -> String;
}

#[derive(Debug, Builder)]
pub struct DiscordOauthProvider {
    client_id: String,
    client_secret: String,
    scopes: Vec<DiscordOauthScope>,
    redirect_url: String,
}

impl OauthProvider for DiscordOauthProvider {
    fn client_id(&self) -> String {
        self.client_id.clone()
    }

    fn client_secret(&self) -> Option<String> {
        Some(self.client_secret.clone())
    }

    fn auth_url(&self) -> String {
        let scopes = self
            .scopes
            .iter()
            .map(|scope| scope.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        format!(
            "https://discord.com/api/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope={}",
            self.client_id(),
            encode(&self.redirect_url()),
            encode(&scopes)
        )
    }

    fn token_url(&self) -> Option<String> {
        Some(String::from("https://discord.com/api/oauth2/token"))
    }

    fn redirect_url(&self) -> String {
        self.redirect_url.clone()
    }
}

#[derive(Debug, Clone, strum_macros::ToString)]
pub enum DiscordOauthScope {
    #[strum(serialize = "identify")]
    Identify,
    #[strum(serialize = "email")]
    Email,
    #[strum(serialize = "connections")]
    Connections,
    #[strum(serialize = "guilds")]
    Guilds,
    #[strum(serialize = "guilds.join")]
    GuildsJoin,
    #[strum(serialize = "gdm.join")]
    GdmJoin,
    #[strum(serialize = "rpc")]
    Rpc,
    #[strum(serialize = "rpc.notifications.read")]
    RpcNotificationsRead,
    #[strum(serialize = "rpc.voice.read")]
    RpcVoiceRead,
    #[strum(serialize = "rpc.voice.write")]
    RpcVoiceWrite,
    #[strum(serialize = "rpc.activities.write")]
    RpcActivitiesWrite,
    #[strum(serialize = "bot")]
    Bot,
    #[strum(serialize = "webhook.incoming")]
    WebhookIncoming,
    #[strum(serialize = "messages.read")]
    MessagesRead,
    #[strum(serialize = "applications.builds.upload")]
    ApplicationsBuildsUpload,
    #[strum(serialize = "applications.builds.read")]
    ApplicationsBuildsRead,
    #[strum(serialize = "applications.commands")]
    ApplicationsCommands,
    #[strum(serialize = "applications.commands.update")]
    ApplicationsCommandsUpdate,
    #[strum(serialize = "applications.store.update")]
    ApplicationsStoreUpdate,
    #[strum(serialize = "applications.entitlements")]
    ApplicationsEntitlements,
    #[strum(serialize = "activities.read")]
    ActivitiesRead,
    #[strum(serialize = "activities.write")]
    ActivitiesWrite,
    #[strum(serialize = "relationships.read")]
    RelationshipsRead,
}
