use std::sync::Arc;

use reqwest::Client;
use serenity::http::Http;

#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> HttpClient {
        let client = Client::builder().build().expect("Failed to build reqwest client");
        HttpClient { client }
    }

    pub fn create_client_from_token(&self, token: &str) -> Http {
        let client = self.client.clone();
        let client = Arc::new(client);
        let token = format!("Bearer {}", token);
        Http::new(client, &token)
    }
}
