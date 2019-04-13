//! The `models` module implements the models of the applications that are
//! persisted in the database.

use actix_web::actix::{Actor, Addr, SyncArbiter, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

/// The `DbExecutor` wraps the connection pool for the database, and implements
/// the `Actor` trait so that it can be used by Actix as part of the application
/// state.
pub struct DbExecutor(Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

/// Create the database connection pool.
///
/// The `DbExecutor` wraps a database connection pool, so that it can be used as
/// an actor in the Actix framework. This method initializes the connection
/// manager as well as the connection pool, and creates a synchronous actor for
/// Actix.
pub fn create_db_executor(database_url: String) -> Addr<DbExecutor> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = diesel::r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    SyncArbiter::start(4, move || DbExecutor(pool.clone()))
}
