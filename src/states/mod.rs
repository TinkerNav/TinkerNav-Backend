mod database_connection;
mod nats_connection;
use crate::config::Config;
pub use database_connection::{establish_connection_pool, PgPool};
use nats::Connection;
pub use nats_connection::establish_connection;
use std::sync::Mutex;

pub struct TNStates {
    pub nats: Mutex<Connection>,
    pub pg_pool: Mutex<PgPool>,
}

impl TNStates {
    pub fn new(config: &Config) -> TNStates {
        let nc = Mutex::new(nats_connection::establish_connection(config));
        let pg_pool = Mutex::new(database_connection::establish_connection_pool(config));
        TNStates { nats: nc, pg_pool }
    }
}
