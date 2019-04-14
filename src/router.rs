//! The `router` module implements a single function that builds a router
//! instance. The router maps HTTP endpoints to `handlers`.

use crate::config::Config;
use crate::handlers::health;
use crate::middleware::diesel::DieselMiddleware;
use crate::models::PgPool;
use gotham::middleware::state::StateMiddleware;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::set::{finalize_pipeline_set, new_pipeline_set, PipelineSet};
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
pub fn router(config: Config, pool: PgPool) -> Router {
    let state = AppState {
        config: Arc::new(config),
    };

    let state_middleware = StateMiddleware::new(state);
    let diesel_middleware = DieselMiddleware::with_pool(pool);

    let pipelines = new_pipeline_set();

    let (pipelines, default) = pipelines.add(
        new_pipeline()
            .add(state_middleware)
            .add(diesel_middleware)
            .build(),
    );

    let pipeline_set = finalize_pipeline_set(pipelines);
    let default_chain = (default, ());

    build_router(default_chain, pipeline_set, |route| {
        route.get_or_head("/_health").to(health::check)
    })
}
