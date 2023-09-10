use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    App, HttpResponse, HttpServer,
};

use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AuthError {
    InvalidUsernameOrPassword,
    InvalidToken,
    CannotRegisterUser,
    BcyptError(bcrypt::BcryptError),
    CannotCreateToken,
}

impl AuthError {
    pub fn as_str(&self) -> &str {
        match *self {
            AuthError::InvalidUsernameOrPassword => "Invalid username or password",
            AuthError::InvalidToken => "Invalid token",
            AuthError::CannotRegisterUser => "Cannot register user",
            AuthError::BcyptError(_) => "Bcrypt error",
            AuthError::CannotCreateToken => "Cannot create token",
        }
    }
}

impl error::ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AuthError::InvalidUsernameOrPassword => StatusCode::UNAUTHORIZED,
            AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthError::CannotRegisterUser => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::BcyptError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::CannotCreateToken => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.as_str().to_string())
    }
}

pub type AuthResult<T> = Result<T, AuthError>;
