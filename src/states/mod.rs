mod database_connection;
mod nats_connection;
use crate::config::Config;
pub use database_connection::{establish_connection_pool, PgPool};
use nats::Connection;
pub use nats_connection::establish_connection;
use std::sync::Mutex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub struct TNStates {
    pub nats: Mutex<Connection>,
    pg_pool: PgPool,
    pub token_generation_key : Mutex<Hmac<Sha256>>, // TODO: Use Lazy here, to initialize only once.
}

impl TNStates {
    pub fn new(config: &Config) -> TNStates {
        let nc = Mutex::new(nats_connection::establish_connection(config));
        let pg_pool = database_connection::establish_connection_pool(config);
        let token_generation_key = Hmac::new_from_slice(config.jwt_secret.as_bytes()).expect("HMAC Key Error");
        TNStates { nats: nc, pg_pool, token_generation_key }
    }

    pub fn get_db_pool(&self) -> PgPool {
        self.pg_pool.clone()
    }
}
