use super::models::Person;
use super::utils::{AuthError, AuthResult};
use crate::states::TNStates;
use actix_web::{
    error,
    web::{Data, Form, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct PersonResponse {
    username: String,
}

pub async fn login(Form(login_data): Form<LoginData>) -> AuthResult<impl Responder> {
    if login_data.username == "admin" {
        return Ok(HttpResponse::Ok().body(format!("Welcome {}!", login_data.username)));
    }
    Err(AuthError::InvalidUsernameOrPassword)
}

pub async fn logout() -> AuthResult<impl Responder> {
    if false {
        return Err(AuthError::InvalidToken);
    }
    Ok(HttpResponse::Ok().body("Logged out"))
}

pub async fn register(states: Data<TNStates>) -> AuthResult<impl Responder> {
    let connection = &mut states.pg_pool.get().unwrap();
    let person = Person::create(connection, "admin", "admin");
    Ok(Json({ PersonResponse { username: person.username } }))
}

pub async fn change_password() -> AuthResult<impl Responder> {
    if false {
        return Err(AuthError::InvalidToken);
    }
    Ok(HttpResponse::Ok().body("Password reset"))
}
