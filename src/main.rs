use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod auth;
mod schema;
mod config;
mod states;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::get();
    let tn_states = states::TNStates::new(&config);

    let connection = web::Data::new(tn_states);
    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .app_data(connection.clone())
            .service(auth::scope())
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
