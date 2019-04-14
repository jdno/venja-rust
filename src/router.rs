//! The `router` module implements a single function that builds a router
//! instance. The router maps HTTP endpoints to `handlers`.

use crate::config::Config;
use crate::handlers::health;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::single::single_pipeline;
use gotham::pipeline::single_middleware;
use gotham::router::builder::*;
use gotham::router::Router;
use std::sync::Arc;

/// The `AppState` is shared across the worker threads. It provides convenient
/// access to the configuration of the application, and the database connection
/// pool.
#[derive(Clone, StateData)]
pub struct AppState {
    /// The configuration of the application
    pub config: Arc<Config>,
}

/// Create a router.
///
/// This function creates a new instance of a router, and maps HTTP endpoints to
/// specific `handlers`.
pub fn router(config: Config) -> Router {
    let state = AppState {
        config: Arc::new(config),
    };

    let middleware = StateMiddleware::new(state);
    let pipeline = single_middleware(middleware);
    let (chain, pipelines) = single_pipeline(pipeline);

    build_router(chain, pipelines, |route| {
        route.get_or_head("/_health").to(health::check)
    })
}
