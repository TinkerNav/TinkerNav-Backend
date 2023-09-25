use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger;
mod auth;
mod schema;
mod config;
mod states;
use config::CONFIG;
use states::STATIC_STATES;

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
    let _ = CONFIG();
    let _ = STATIC_STATES();
    let tn_states = states::TNStates::new(CONFIG());

    let connection = web::Data::new(tn_states);
    // access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .app_data(connection.clone())
            .service(auth::scope())
    })
    .bind((CONFIG().host.clone(), CONFIG().port))?
    .run()
    .await
}
