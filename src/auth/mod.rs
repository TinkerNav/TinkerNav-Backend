mod login;
mod utils;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use login::login;

pub fn config(cfg: &mut web::ServiceConfig) {
    web::resource("/login")
        .route(web::post().to(login));
}
