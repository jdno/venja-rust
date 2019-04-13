//! The `router` module implements a single function that builds a router
//! instance. The router maps HTTP endpoints to `handlers`.

use crate::handlers::health;
use gotham::router::builder::*;
use gotham::router::Router;

/// Create a router.
///
/// This function creates a new instance of a router, and maps HTTP endpoints to
/// specific `handlers`.
pub fn router() -> Router {
    build_simple_router(|route| route.get_or_head("/_health").to(health::index))
}
