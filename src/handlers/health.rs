//! The `health` module implements an endpoint that can be used to monitor the
//! application and its health. The endpoint checks the app itself, as well as
//! the services it depends on, and returns a detailed status report.

use crate::router::AppState;
use gotham::state::{FromState, State};

/// The health endpoint has a single action that checks if the application, and
/// the services it depends on, work correctly.
pub fn check(state: State) -> (State, String) {
    let message = {
        let app_state = AppState::borrow_from(&state);
        format!(
            "Running in environment {}",
            app_state.config.env.to_string()
        )
    };

    (state, message)
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, Environment};
    use crate::router::router;
    use gotham::test::{TestResponse, TestServer};
    use hyper::StatusCode;

    fn request_check(endpoint: String) -> TestResponse {
        let config = Config {
            env: Environment::Test,
            ..Default::default()
        };
        let address = format!("http://{}/{}", config.server_address(), endpoint);
        let test_server = TestServer::new(router(config)).unwrap();

        test_server.client().get(address).perform().unwrap()
    }

    #[test]
    fn check_status_ok() {
        let response = request_check(String::from("_health"));
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn check_body() {
        let response = request_check(String::from("_health"));

        let body = response.read_body().unwrap();
        let expected = format!("Running in environment test");

        assert_eq!(&body[..], expected.as_bytes());
    }
}
