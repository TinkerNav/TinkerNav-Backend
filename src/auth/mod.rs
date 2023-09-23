mod login;
mod errors;
mod models;
mod redir_middleware;
mod token;
mod api;

use actix_web::web;
use api::{create_bot, delete_bot};
use login::{login, logout, register};

pub fn scope() -> actix_web::Scope {
    web::scope("/auth")
        .service(web::resource("/person/login").route(web::post().to(login)))
        .service(web::resource("/person/logout").route(web::post().to(logout)))
        .service(web::resource("/person/registry").route(web::post().to(register)))
        .service(web::resource("/bot/registry").route(web::post().to(api::create_bot)))
        .service(web::resource("/bot/registry").route(web::delete().to(api::delete_bot)))
        .service(web::resource("/bot/token").route(web::post().to(api::create_bot_token)))
        .service(web::resource("/bot/token").route(web::patch().to(api::rotate_bot_token)))
        .service(web::resource("/bot/token").route(web::delete().to(api::delete_bot_token)))
}
