use crate::{
    data::{authorize, HttpClient},
    error::AppError,
};
use actix_session::Session;
use actix_web::{web, HttpResponse};

pub(crate) async fn me(
    session: Session,
    client: web::Data<HttpClient>,
) -> Result<HttpResponse, AppError> {
    let token = authorize(&session)?;
    let http = client.create_client_from_token(&token.access_token);

    let user = http.get_current_user().await?;

    Ok(HttpResponse::Ok().json(user))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::HttpClient;
    use actix_session::UserSession;
    use actix_web::{dev::ServiceResponse, test, web, ResponseError};
    use dotenv::dotenv;
    use serenity::model::user::CurrentUser;
    use std::env;

    #[actix_rt::test]
    async fn test_with_valid_token() {
        // Load Credentials from environment
        dotenv().ok();
        let access_token = env::var("API_ACCESS_TOKEN").expect("API_ACCESS_TOKEN is not set");
        let refresh_token = env::var("API_REFRESH_TOKEN").expect("API_REFRESH_TOKEN is not set");

        // Build Request
        let req = test::TestRequest::default().to_http_request();
        req.get_session()
            .set("access_token", &access_token)
            .unwrap();
        req.get_session()
            .set("refresh_token", &refresh_token)
            .unwrap();
        let client = HttpClient::new();

        // Send Request
        let resp = me(req.get_session(), web::Data::new(client))
            .await
            .ok()
            .unwrap();
        assert_eq!(resp.status().as_u16(), 200);

        // Check Body
        let srv = ServiceResponse::new(req, resp);
        let data: CurrentUser = test::read_body_json(srv).await;
        assert_eq!(data.id.0, 334017809090740224);
    }

    #[actix_rt::test]
    async fn test_with_invalid_token() {
        // Load Credentials from environment
        dotenv().ok();
        let access_token = format!("aaa");
        let refresh_token = format!("peco");

        // Build Request
        let req = test::TestRequest::default().to_http_request();
        req.get_session()
            .set("access_token", &access_token)
            .unwrap();
        req.get_session()
            .set("refresh_token", &refresh_token)
            .unwrap();
        let client = HttpClient::new();

        // Send Request
        let resp = me(req.get_session(), web::Data::new(client))
            .await
            .err()
            .unwrap();
        assert_eq!(resp.status_code().as_u16(), 401);
        assert_eq!(resp.to_string(), "Discord returned an error".to_string());
    }

    #[actix_rt::test]
    async fn test_without_token() {
        // Build Request
        let req = test::TestRequest::default().to_http_request();
        let client = HttpClient::new();

        // Send Request
        let resp = me(req.get_session(), web::Data::new(client))
            .await
            .err()
            .unwrap();
        assert_eq!(resp.status_code().as_u16(), 401);
        assert_eq!(resp.to_string(), "Token is missing".to_string());
    }
}
