use crate::{data::{authorize, HttpClient}, error::AppError};
use actix_session::Session;
use actix_web::{get, HttpResponse, web};

#[get("/users/@me")]
pub(crate) async fn me(
    session: Session,
    client: web::Data<HttpClient>,
) -> Result<HttpResponse, AppError> {
    let token = authorize(&session)?;
    let http = client.create_client_from_token(&token.access_token);

    let user = http.get_current_user().await?;

    Ok(HttpResponse::Ok().json(user))
}
