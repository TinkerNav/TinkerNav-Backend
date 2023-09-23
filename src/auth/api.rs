use super::errors::{AuthError, AuthResult};
use super::models::{User, Bot, BotApiToken};
use super::token::{generate_cookie, generate_token};
use crate::states::TNStates;
use actix_web::{
    web::{Data, Form, Json},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize)]
pub struct CreateBot {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct ReferBot {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct BotResponse {
    pub name: String,
    pub description: String,
}

pub async fn create_bot(
    form: Form<CreateBot>,
    states: Data<TNStates>,
) -> AuthResult<impl Responder> {
    let connection = &mut states.get_db_pool().get().unwrap();
    let bot = Bot::create(connection, &form.name, &form.description)?;
    Ok(Json(BotResponse {
        name: bot.name,
        description: bot.description,
    }))
}

pub async fn delete_bot(
    form: Form<ReferBot>,
    states: Data<TNStates>,
) -> AuthResult<impl Responder> {
    let connection = &mut states.get_db_pool().get().unwrap();
    let bot = Bot::find_name(connection, &form.name).ok_or(AuthError::UserNotFound)?;
    bot.delete(connection)?;
    Ok(Json(BotResponse {
        name: bot.name,
        description: bot.description,
    }))
}

async fn authenticate(
    states: Data<TNStates>,
    token: &String,
) -> AuthResult<Bot> {
    let connection = &mut states.get_db_pool().get().unwrap();
    let bot_api_token = BotApiToken::find_token(connection, token).ok_or(AuthError::InvalidToken)?;
    let bot = Bot::find(connection, bot_api_token.bot_uuid).ok_or(AuthError::InvalidToken)?;
    Ok(bot)
}