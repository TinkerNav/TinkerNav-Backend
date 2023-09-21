mod login;
mod errors;
mod models;
mod redir_middleware;
mod token;
mod api;

use actix_web::web;
use login::{login, logout, register};
use api::{create_bot, delete_bot};

pub fn scope() -> actix_web::Scope {
    web::scope("/auth")
        .service(web::resource("/person/login").route(web::post().to(login)))
        .service(web::resource("/person/logout").route(web::post().to(logout)))
        .service(web::resource("/person").route(web::post().to(register)))
        .service(web::resource("/bot").route(web::post().to(api::create_bot)))
        .service(web::resource("/bot").route(web::delete().to(api::delete_bot)))
}
