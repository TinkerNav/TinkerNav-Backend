mod database_connection;
mod nats_connection;
pub use database_connection::{establish_connection_pool, PgPool};
use nats::Connection;
pub use nats_connection::establish_connection;

pub struct TNStates {
    pub nats: Connection,
    pub pg_pool: PgPool,
}

impl TNStates {
    pub fn new() -> TNStates {
        let nc: nats::Connection = nats_connection::establish_connection();
        let pg_pool = database_connection::establish_connection_pool();
        TNStates { nats: nc, pg_pool }
    }
}
