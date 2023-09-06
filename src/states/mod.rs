mod database_connection;
mod nats_connection;
use crate::config::Config;
pub use database_connection::{establish_connection_pool, PgPool};
use nats::Connection;
pub use nats_connection::establish_connection;

pub struct TNStates {
    pub nats: Connection,
    pub pg_pool: PgPool,
}

impl TNStates {
    pub fn new(config: &Config) -> TNStates {
        let nc: nats::Connection = nats_connection::establish_connection(config);
        let pg_pool = database_connection::establish_connection_pool(config);
        TNStates { nats: nc, pg_pool }
    }
}
