use nats::Connection;
use std::env;
use crate::config::Config;
pub fn establish_connection(config: &Config) -> Connection {
    let nats_url = config.nats_url.as_str();
    nats::connect(nats_url).expect("Failed to connect to NATS")
}
