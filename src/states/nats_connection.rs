use dotenv::dotenv;
use nats::Connection;
use std::env;
pub fn establish_connection() -> Connection {
    dotenv().ok();

    let nats_url = env::var("NATS_URL").expect("NATS_URL must be set");
    nats::connect(nats_url).expect("Failed to connect to NATS")
}
