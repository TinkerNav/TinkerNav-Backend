use super::errors::{AuthError, AuthResult};
use super::models::User;
use crate::config::CONFIG;
use actix_web::cookie::Cookie;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use uuid::Uuid;
use actix_web::web::Data;
use crate::states::STATIC_STATES;
use crate::states::TNStates;

pub fn generate_token(states: &Data<TNStates>, user: &dyn User) -> AuthResult<String> {
    let key = &STATIC_STATES.token_generation_key;
    let mut claims = BTreeMap::new();
    claims.insert("uuid", user.get_uuid().to_string());
    let token_str = claims.sign_with_key(key).map_err(|_| AuthError::CannotCreateToken)?;
    Ok(token_str)
}

pub fn current_user_person(token: String) -> AuthResult<Uuid> {
    let key = &STATIC_STATES.token_generation_key;
    let claims: BTreeMap<String, String> =
        token.verify_with_key(key).map_err(|_| AuthError::InvalidToken)?;
    let uuid = claims.get("uuid").ok_or(AuthError::InvalidToken)?;
    Uuid::parse_str(uuid).map_err(|_| AuthError::InvalidToken)
}

pub fn generate_cookie(token: String) -> Cookie<'static> {
    Cookie::build("token", token).secure(true).http_only(true).finish()
}
