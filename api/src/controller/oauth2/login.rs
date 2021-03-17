use crate::{data::OauthClient, error::AppError};
use actix_session::Session;
use actix_web::{http::header, web, HttpResponse};

pub(crate) async fn login(
    session: Session,
    oauth: web::Data<OauthClient>,
) -> Result<HttpResponse, AppError> {
    let (url, csrf_token, pkce_verifier) = oauth.as_ref().generate_auth_url();
    session.set("csrf_token", csrf_token)?;
    session.set("pkce_verifier", pkce_verifier)?;
    Ok(HttpResponse::Found().header(header::LOCATION, url).finish())
}
