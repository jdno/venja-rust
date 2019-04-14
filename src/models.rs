//! The `models` module implements the models of the applications that are
//! persisted in the database.

use crate::middleware::diesel::Diesel;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::Connection;
use gotham::state::FromState;
use gotham::state::State;
use r2d2::PooledConnection;

/// The `PgPool` aliases a connection pool for Postgres that is managed by r2d2.
pub type PgPool = Pool<ConnectionManager<PgConnection>>;
/// The `PgConn` aliases a pooled Postgres connection that is managed by r2d2.
pub type PgConn = PooledConnection<ConnectionManager<PgConnection>>;

/// Create a connection pool.
pub fn connection_pool(database_url: String) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

/// Retrieve a connection from the shared state.
pub fn connection<T>(state: &State) -> PooledConnection<ConnectionManager<T>>
where
    T: Connection + 'static,
{
    Diesel::borrow_from(state)
        .conn()
        .expect("Did not obtain valid Diesel connection from R2D2 pool")
}
