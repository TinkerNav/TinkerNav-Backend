mod login;
mod utils;
mod models;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use login::{login, logout};

pub fn scope() -> actix_web::Scope {
    web::scope("/auth")
        .service(web::resource("/login").route(web::post().to(login)))
        .service(web::resource("/logout").route(web::post().to(logout)))
}
