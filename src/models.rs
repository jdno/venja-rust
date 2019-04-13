//! The `models` module implements the models of the applications that are
//! persisted in the database.

use actix_web::actix::{Actor, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

/// The `DbExecutor` wraps the connection pool for the database, and implements
/// the `Actor` trait so that it can be used by Actix as part of the application
/// state.
pub struct DbExecutor(Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}
