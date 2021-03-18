use std::sync::Arc;

use reqwest::Client;
use serenity::http::Http;

#[derive(Debug, Clone)]
pub struct HttpClient {
    pub client: Arc<Client>,
}

impl HttpClient {
    pub fn new() -> HttpClient {
        let client = Client::builder()
            .build()
            .expect("Failed to build reqwest client");
        let client = Arc::new(client);
        HttpClient { client }
    }

    pub fn create_client_from_token(&self, token: &str) -> Http {
        let client = self.client.clone();
        let token = format!("Bearer {}", token);
        Http::new(client, &token)
    }
}
