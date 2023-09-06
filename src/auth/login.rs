use super::utils::{AuthenticationError, AuthenticationResult};
use actix_web::{error, web::Form, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

pub async fn login(Form(login_data): Form<LoginData>) -> AuthenticationResult<impl Responder> {
    if login_data.username == "admin" {
        return Ok(HttpResponse::Ok().body(format!("Welcome {}!", login_data.username)));
    }
    Err(AuthenticationError::InvalidUsernameOrPassword)
}

pub async fn logout() -> AuthenticationResult<impl Responder> {
    if false {
        return Err(AuthenticationError::InvalidToken);
    }
    Ok(HttpResponse::Ok().body("Logged out"))
}
