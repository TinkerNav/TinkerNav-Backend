use super::errors::{AuthError, AuthResult};
use super::models::User;
use actix_web::cookie::Cookie;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use uuid::Uuid;

pub fn generate_token(user: &dyn User) -> AuthResult<String> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").expect("HMAC Key Error");
    let mut claims = BTreeMap::new();
    claims.insert("uuid", user.get_uuid().to_string());
    let token_str = claims.sign_with_key(&key).map_err(|_| AuthError::CannotCreateToken)?;
    Ok(token_str)
}

pub fn current_user_person(_token: String) -> AuthResult<Uuid> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").expect("HMAC Key Error");
    let token_str =
        "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0";
    let claims: BTreeMap<String, String> =
        token_str.verify_with_key(&key).map_err(|_| AuthError::InvalidToken)?;
    let uuid = claims.get("uuid").ok_or(AuthError::InvalidToken)?;
    Uuid::parse_str(uuid).map_err(|_| AuthError::InvalidToken)
}

pub fn generate_cookie(token: String) -> Cookie<'static> {
    Cookie::build("token", token).secure(true).http_only(true).finish()
}
