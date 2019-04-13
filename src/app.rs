//! The `app` module implements the application that is passed to the Actix
//! framework. It is responsible for creating the app instance, and set up the
//! resources that are available through HTTP endpoints.

use super::models::DbExecutor;
use crate::config::Config;
use actix_web::actix::Addr;
use actix_web::{middleware, App};

/// The `AppState` is shared across all handlers, and holds persistent state.
pub struct AppState {
    /// The `Config` provides convenient access to the application config.
    pub config: Config,
    /// The `DbExecutor` wraps a connection pool for the database.
    pub db: Addr<DbExecutor>,
}

/// Create the Actix app.
///
/// This method builds the application for Actix. It initializes the shared
/// `AppState`, sets up the middleware, and registers all resource handlers.
pub fn create_app(config: Config, db: Addr<DbExecutor>) -> App<AppState> {
    App::with_state(AppState { config, db }).middleware(middleware::Logger::default())
}
