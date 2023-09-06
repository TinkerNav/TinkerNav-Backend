use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    App, HttpResponse, HttpServer,
};

use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum AuthenticationError {
    InvalidUsernameOrPassword,
    InvalidToken,
}

impl AuthenticationError {
    pub fn as_str(&self) -> &str {
        match *self {
            AuthenticationError::InvalidUsernameOrPassword => "Invalid username or password",
            AuthenticationError::InvalidToken => "Invalid token",
        }
    }
}

impl error::ResponseError for AuthenticationError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AuthenticationError::InvalidUsernameOrPassword => StatusCode::UNAUTHORIZED,
            AuthenticationError::InvalidToken => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.as_str().to_string())
    }
}

pub type AuthenticationResult<T> = Result<T, AuthenticationError>;
