mod database_connection;
mod nats_connection;
use crate::config::Config;
pub use database_connection::{establish_connection_pool, PgPool};
use nats::Connection;
pub use nats_connection::establish_connection;
use std::sync::Mutex;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use once_cell::sync::OnceCell;

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

impl StaticTNStates {
    pub fn new() -> StaticTNStates{
        let key = Hmac::<Sha256>::new_from_slice(b"secret").unwrap();
        StaticTNStates { token_generation_key: key }
    }
}

static _STATIC_STATES: OnceCell<StaticTNStates> = OnceCell::new();

pub fn STATIC_STATES() -> &'static StaticTNStates {
    _STATIC_STATES.get_or_init(|| StaticTNStates::new())
}