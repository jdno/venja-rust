// This module has been copied from https://github.com/gotham-rs/gotham, since
// it is not available in the published crate. Work on the module has stopped,
// and https://github.com/gotham-rs/gotham/issues/309 explores an async
// approach.
// FIXME: Remove this module once Gotham has official support for Diesel

//! Makes a Diesel connection available to Middleware and Handlers that are involved in
//! processing a Request.
//!
//! Utilises r2d2 pooling to ensure efficent database usage and prevent resource exhaustion.

#![allow(missing_docs)]
use std::io;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process;

use futures::{future, Future};

use gotham::handler::HandlerFuture;
use gotham::middleware::{Middleware, NewMiddleware};
use gotham::state::{request_id, State};

use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::Connection;
use r2d2::Error;

#[derive(StateData)]
pub struct Diesel<T>
where
    T: Connection + 'static,
{
    pool: Pool<ConnectionManager<T>>,
}

impl<T> Diesel<T>
where
    T: Connection + 'static,
{
    pub(crate) fn new(pool: Pool<ConnectionManager<T>>) -> Self {
        Diesel { pool }
    }

    /// Provides access to a Diesel connection from our r2d2 backed connection pool.
    pub fn conn(&self) -> Result<PooledConnection<ConnectionManager<T>>, Error> {
        self.pool.get()
    }
}

/// A Gotham compatible Middleware that manages a pool of Diesel connections via r2d2 and hands
/// out connections to other Middleware and Handlers that require them via the Gotham `State`
/// mechanism.
pub struct DieselMiddleware<T>
where
    T: Connection + 'static,
{
    pool: AssertUnwindSafe<Pool<ConnectionManager<T>>>,
}

/// Instance created by DieselMiddleware for each request that implements
/// the actual logic of the middleware.
pub struct DieselMiddlewareImpl<T>
where
    T: Connection + 'static,
{
    pool: Pool<ConnectionManager<T>>,
}

impl<T> DieselMiddleware<T>
where
    T: Connection,
{
    /// Sets up a new instance of the middleware and establishes a connection to the database.
    ///
    /// * The database to connect to, including authentication components.
    ///
    /// # Panics
    /// If the database identified in `database_url` cannot be connected to at application start.
    ///
    /// n.b. connection will be re-established if the database goes away and returns mid execution
    /// without panic.
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<T>::new(database_url);

        let pool = Pool::<ConnectionManager<T>>::new(manager).expect("Failed to create pool.");

        DieselMiddleware::with_pool(pool)
    }

    /// Sets up a new instance of the middleware and establishes a connection to the database.
    ///
    /// * The connection pool (with custom configuration)
    ///
    /// n.b. connection will be re-established if the database goes away and returns mid execution
    /// without panic.
    pub fn with_pool(pool: Pool<ConnectionManager<T>>) -> Self {
        DieselMiddleware {
            pool: AssertUnwindSafe(pool),
        }
    }
}

impl<T> NewMiddleware for DieselMiddleware<T>
where
    T: Connection + 'static,
{
    type Instance = DieselMiddlewareImpl<T>;

    fn new_middleware(&self) -> io::Result<Self::Instance> {
        match catch_unwind(|| self.pool.clone()) {
            Ok(pool) => Ok(DieselMiddlewareImpl { pool }),
            Err(_) => {
                error!(
                    "PANIC: r2d2::Pool::clone caused a panic, unable to rescue with a HTTP error"
                );
                eprintln!(
                    "PANIC: r2d2::Pool::clone caused a panic, unable to rescue with a HTTP error"
                );
                process::abort()
            }
        }
    }
}

impl<T> Clone for DieselMiddleware<T>
where
    T: Connection + 'static,
{
    fn clone(&self) -> Self {
        match catch_unwind(|| self.pool.clone()) {
            Ok(pool) => DieselMiddleware {
                pool: AssertUnwindSafe(pool),
            },
            Err(_) => {
                error!("PANIC: r2d2::Pool::clone caused a panic");
                eprintln!("PANIC: r2d2::Pool::clone caused a panic");
                process::abort()
            }
        }
    }
}

impl<T> Middleware for DieselMiddlewareImpl<T>
where
    T: Connection + 'static,
{
    fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
    where
        Chain: FnOnce(State) -> Box<HandlerFuture>,
    {
        trace!("[{}] pre chain", request_id(&state));
        state.put(Diesel::<T>::new(self.pool));

        let f = chain(state).and_then(move |(state, response)| {
            {
                trace!("[{}] post chain", request_id(&state));
            }
            future::ok((state, response))
        });
        Box::new(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use diesel::pg::PgConnection;
    use diesel::r2d2::{ConnectionManager, Pool};

    static DATABASE_URL: &'static str = "postgres://localhost/venja_test";

    #[test]
    fn new_with_default_config() {
        let manager = ConnectionManager::new(DATABASE_URL);
        let pool = Pool::<ConnectionManager<PgConnection>>::new(manager).unwrap();
        let _middleware = DieselMiddleware::with_pool(pool);
    }

    #[test]
    fn new_with_custom_pool_config() {
        let manager = ConnectionManager::new(DATABASE_URL);
        let pool = Pool::<ConnectionManager<PgConnection>>::builder()
            .min_idle(Some(1))
            .build(manager)
            .unwrap();
        let _middleware = DieselMiddleware::with_pool(pool);
    }
}
