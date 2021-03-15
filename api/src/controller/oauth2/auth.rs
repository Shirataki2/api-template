use std::env;

use crate::{data::OauthClient, error::AppError};
use actix_session::Session;
use actix_web::{get, web, HttpResponse, http::header};
use oauth2::{AsyncCodeTokenRequest, AuthorizationCode, TokenResponse, reqwest::async_http_client};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[get("/auth")]
pub(crate) async fn auth(
    session: Session,
    oauth: web::Data<OauthClient>,
    query: web::Query<AuthRequest>,
) -> Result<HttpResponse, AppError> {
    let code = AuthorizationCode::new(query.code.clone());

    let verifier = match session.get("pkce_verifier")? {
        Some(verifier) => verifier,
        None => {
            error!("PKCE verifier not found!");
            return Err(AppError::InternalServerError);
        },
    };

    let token = oauth
        .client
        .exchange_code(code)
        .set_pkce_verifier(verifier)
        .request_async(async_http_client)
        .await;

    let token = match token {
        Ok(token) => token,
        Err(_) => return Err(AppError::AuthorizationServerError)
    };

    if let Some(refresh_token) = token.refresh_token() {
        session.set("refresh_token", refresh_token.secret())?;
        session.set("access_token", token.access_token().secret())?;
    } else {
        return Err(AppError::TokenRegistrationError)
    }
    let url = env::var("FRONTEND_URL").unwrap_or("http://localhost:3000".to_string());
    Ok(HttpResponse::Found().header(header::LOCATION, url).finish())
}
