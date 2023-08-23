mod states;
use states::TNStates;
mod user;
use rocket::*;

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
    connections.nats.subscribe("foo.*").unwrap().with_handler(move |msg| {
        println!("HHHHHHHHHHHHHHHHHHHHHH Received {}", &msg);
        Ok(())
    });
    connections.nats.publish("foo.no", "JSON").expect("Failed to publish");
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    setup_logger().expect("Failed to setup logger");
    let states = TNStates::new();
    rocket::build()
        .mount("/", routes![index])
        .mount("/person", routes![user::register])
        .manage(states)
}
