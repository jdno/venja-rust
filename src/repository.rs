//! The `respository` module implements the interaction with the database.

use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Builder, CustomizeConnection, Error, Pool, PooledConnection};

/// The `PgPool` is a connection pool for connections to the Postgres database.
pub type PgPool = Pool<ConnectionManager<PgConnection>>;

/// The `Connection` type describes a pooled connection to the database through
/// the connection pool.
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

/// The `Repository` is a wrapper around the database. It manages a connection
/// pool that can be borrowed from to query the database.
pub struct Repository {
    connection_pool: PgPool,
}

impl Repository {
    /// Initialize a new repository and connect to the given database.
    pub fn new(database_url: &str) -> Self {
        Self::from_pool_builder(database_url, Builder::default())
    }

    /// Initialize a new repository for testing, which isolates queries in transactions.
    pub fn with_test_transactions(database_url: &str) -> Self {
        let customizer = TestConnectionCustomizer {};
        let builder = Pool::builder().connection_customizer(Box::new(customizer));
        Self::from_pool_builder(database_url, builder)
    }

    /// Get a connection from the managed connection pool.
    pub fn connection(&self) -> Result<Connection, Error> {
        self.connection_pool.get()
    }

    fn from_pool_builder(
        database_url: &str,
        builder: Builder<ConnectionManager<PgConnection>>,
    ) -> Self {
        let manager = ConnectionManager::new(database_url);
        let connection_pool = builder
            .build(manager)
            .expect("could not initiate test db pool");
        Self { connection_pool }
    }
}

impl Clone for Repository {
    fn clone(&self) -> Self {
        Self {
            connection_pool: self.connection_pool.clone(),
        }
    }
}

#[derive(Debug)]
struct TestConnectionCustomizer;

impl<C, E> CustomizeConnection<C, E> for TestConnectionCustomizer
where
    C: diesel::connection::Connection,
    E: std::error::Error + Sync + Send,
{
    fn on_acquire(&self, conn: &mut C) -> Result<(), E> {
        match conn.begin_test_transaction() {
            Ok(_) => Ok(()),
            Err(_) => Ok(()),
        }
    }
}
