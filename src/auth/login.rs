use actix_web::{web::Form, Responder, HttpResponse, error};
use serde::Deserialize;
use super::utils::AuthenticationError;

#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}


pub async fn login(Form(login_data): Form<LoginData>) -> Result<impl Responder, impl error::ResponseError> {
    if login_data.username == "admin" {
        return Ok(HttpResponse::Ok().body(format!("Welcome {}!", login_data.username)))
    }
    Err(AuthenticationError::InvalidUsernameOrPassword)
}

pub async fn logout() -> Result<impl Responder, impl error::ResponseError> {
    if false {
        return Err(AuthenticationError::InvalidToken)
    }
    Ok(HttpResponse::Ok().body("Logged out"))
}
