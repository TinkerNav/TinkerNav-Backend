mod login;
mod errors;
mod models;
mod redir_middleware;
mod token;
mod api;

use actix_web::web;
use login::{login, logout, register};

pub fn scope() -> actix_web::Scope {
    web::scope("/auth")
        .service(web::resource("/login").route(web::post().to(login)))
        .service(web::resource("/logout").route(web::post().to(logout)))
        .service(web::resource("/user").route(web::post().to(register)))
}
