use crate::config::Config;
use nats::Connection;
use std::env;
pub fn establish_connection(config: &Config) -> Connection {
    let nats_url = config.nats_url.as_str();
    nats::connect(nats_url).expect("Failed to connect to NATS")
}
