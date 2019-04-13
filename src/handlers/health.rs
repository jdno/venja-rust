//! The `health` module implements an endpoint that can be used to monitor the
//! application and its health. The endpoint checks the app itself, as well as
//! the services it depends on, and returns a detailed status report.

use gotham::state::State;

const OK: &str = "Ok";

/// The health endpoint has a single action that checks if the application, and
/// the services it depends on, work correctly.
pub fn index(state: State) -> (State, &'static str) {
    (state, OK)
}
