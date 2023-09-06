use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use std::env;
use crate::config::Config;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub fn establish_connection_pool(config: &Config) -> Pool<ConnectionManager<PgConnection>> {

    let database_url = config.postgres_url.as_str();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder().test_on_check_out(true).build(manager).expect("Could not build connection pool")
}
