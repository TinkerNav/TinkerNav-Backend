mod database_connection;
mod nats_connection;
use crate::config::Config;
pub use database_connection::{establish_connection_pool, PgPool};
use nats::Connection;
pub use nats_connection::establish_connection;
use std::sync::Mutex;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use lazy_static::lazy_static;

// TNStates manages the general states between the accesses.
// Note that TN Backend is generally a STATELESS application and the TNStates here only manages the necessary states (such as DB connection pools).
pub struct TNStates {
    pub nats: Mutex<Connection>,
    pg_pool: PgPool,
}

impl TNStates {
    pub fn new(config: &Config) -> TNStates {
        let nc = Mutex::new(nats_connection::establish_connection(config));
        let pg_pool = database_connection::establish_connection_pool(config);
        TNStates { nats: nc, pg_pool }
    }

    pub fn get_db_pool(&self) -> PgPool {
        self.pg_pool.clone()
    }
}

pub struct StaticTNStates {
    pub token_generation_key: Hmac<Sha256>,
}

lazy_static! {
    pub static ref STATIC_STATES: StaticTNStates = StaticTNStates {
        token_generation_key: Hmac::new_from_slice(&Config::get().jwt_secret.as_bytes()).expect("HMAC can take key of any size")
    };
}