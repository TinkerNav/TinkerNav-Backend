use super::errors::{AuthError, AuthResult};
use super::models::Person;
use super::token::{generate_cookie, generate_token};
use crate::states::TNStates;
use actix_web::{
    web::{Data, Form, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginData {
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct RegisterData {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct PersonResponse {
    username: String,
}

pub async fn login(
    states: Data<TNStates>, Form(login_data): Form<LoginData>,
) -> AuthResult<impl Responder> {
    let username = login_data.username;
    let password = login_data.password;
    let connection = &mut states.get_db_pool().get().unwrap();
    let person = Person::login(connection, &username, &password)?;
    let token = generate_token(&person)?;
    let cookie = generate_cookie(token);
    Ok(HttpResponse::Ok().cookie(cookie).body("Logged in"))
}

pub async fn logout() -> AuthResult<impl Responder> {
    if false {
        return Err(AuthError::InvalidToken);
    }
    Ok(HttpResponse::Ok().body("Logged out"))
}

pub async fn register(
    states: Data<TNStates>, Form(register_data): Form<RegisterData>,
) -> AuthResult<impl Responder> {
    let connection = &mut states.get_db_pool().get().unwrap();
    let username = register_data.username;
    let password = register_data.password;
    let person = Person::create(connection, &username, &password)?;
    Ok(Json(PersonResponse { username: person.username }))
}

pub async fn change_password() -> AuthResult<impl Responder> {
    if false {
        return Err(AuthError::InvalidToken);
    }
    Ok(HttpResponse::Ok().body("Password reset"))
}
