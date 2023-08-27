mod states;
mod bots;
mod type_sys;
mod user;
mod config;
use config::Config;
use rocket::*;
use states::TNStates;

pub mod schema;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(::log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

#[get("/")]
fn index(connections: &State<TNStates>) -> &'static str {
    connections.nats.publish("foo.no", "JSON").expect("Failed to publish");
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let config = Config::get();
    let figment = rocket::Config::figment()
        .merge(("address", config.host))
        .merge(("port", config.port))
        .merge(("workers", config.workers))
        .merge(("secret_key", config.secret_key));

    setup_logger().expect("Failed to setup logger");
    let states = TNStates::new();
    rocket::custom(figment)
        .mount("/", routes![index])
        .mount("/person", routes![user::register, user::login, user::logout])
        .mount("/bot", routes![bots::test])
        .manage(states)
}
