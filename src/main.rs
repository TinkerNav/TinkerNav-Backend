use nats::Connection;
use rocket::*;

pub mod schema;
mod user;
mod database;

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

struct TNStates {
    nats: Connection,
}
#[get("/")]
fn index(connections: &State<TNStates>) -> &'static str {
    connections.nats.publish("foo", "Hello World!").expect("Failed to publish");
    let connection = &mut database::establish_connection();
    let new_user = user::User::create(connection, "test".to_string(), "test".to_string());
    println!("New user: {:?}", new_user);
    let check_user = user::User::get(connection, new_user.uuid);
    println!("Check user: {:?}", check_user);
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    setup_logger().expect("Failed to setup logger");
    let nc: nats::Connection = nats::connect("demo.nats.io").expect("Failed to connect to NATS");
    let connections = TNStates { nats: nc };
    rocket::build().mount("/", routes![index]).manage(connections)
}
