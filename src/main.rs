#[macro_use] extern crate rocket;
use rocket::State;
use nats::Connection;

struct TNStates {
    nats: Connection
}
#[get("/")]
fn index(connections: &State<TNStates>) -> &'static str {
    connections.nats.publish("foo", "Hello World!").expect("Failed to publish");
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let nc : nats::Connection= nats::connect("demo.nats.io").expect("Failed to connect to NATS");
    let connections = TNStates { nats: nc };
    rocket::build().mount("/", routes![index]).manage(connections)
}
