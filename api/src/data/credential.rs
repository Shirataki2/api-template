use actix_session::Session;

use crate::error::AppError;

pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn authorize(session: &Session) -> Result<Token, AppError> {
    let access_token  = match session.get::<String>("access_token")? {
        Some(access_token) => access_token,
        None => return Err(AppError::Unauthorized("Token is missing".to_string())),
    };
    let refresh_token = match session.get::<String>("refresh_token")? {
        Some(refresh_token) => refresh_token,
        None => return Err(AppError::Unauthorized("Token is missing".to_string())),
    };
    Ok(Token { access_token, refresh_token})
}