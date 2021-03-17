use std::env;

use crate::error::AppError;
use actix_session::Session;
use actix_web::{http::header, HttpResponse};

pub(crate) async fn logout(session: Session) -> Result<HttpResponse, AppError> {
    session.clear();
    let url = env::var("FRONTEND_URL").unwrap_or("http://localhost:3000".to_string());
    Ok(HttpResponse::Found().header(header::LOCATION, url).finish())
}
