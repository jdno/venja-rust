//! The `health` module implements an endpoint that can be used to monitor the
//! application and its health. The endpoint checks the app itself, as well as
//! the services it depends on, and returns a detailed status report.

use crate::config::Environment;
use crate::router::{AppState, Repository};
use diesel::query_dsl::RunQueryDsl;
use diesel::sql_query;
use futures::future;
use futures::future::Future;
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::helpers::http::response::create_response;
use gotham::state::{FromState, State};
use hyper::StatusCode;
use serde::Serialize;

/// https://tools.ietf.org/html/draft-inadarei-api-health-check-02
#[derive(Serialize)]
enum Status {
    Pass,
    Fail,
}

#[derive(Serialize)]
struct Health {
    environment: Environment,
    postgres: Status,
}

/// The health endpoint has a single action that checks if the application, and
/// the services it depends on, work correctly.
pub fn check(state: State) -> Box<HandlerFuture> {
    let app_state = AppState::borrow_from(&state).clone();
    let repo = Repository::borrow_from(&state).clone();

    let future = repo
        .run(move |connection| sql_query("SELECT 1").execute(&connection))
        .map_err(IntoHandlerError::into_handler_error)
        .then(move |result| {
            let postgres_status = match result {
                Ok(_) => Status::Pass,
                Err(_) => Status::Fail,
            };

            let health = Health {
                environment: app_state.config.env.clone(),
                postgres: postgres_status,
            };

            let response = create_response(
                &state,
                StatusCode::OK,
                mime::APPLICATION_JSON,
                serde_json::to_string(&health).expect("Failed to serialize health"),
            );

            future::ok((state, response))
        });

    Box::new(future)
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, Environment};
    use crate::handlers::health::Health;
    use crate::handlers::health::Status::Pass;
    use crate::router::{router, Repository};
    use gotham::test::{TestResponse, TestServer};
    use hyper::StatusCode;
    use std::str;

    fn request_check(endpoint: String) -> TestResponse {
        let config = Config {
            env: Environment::Test,
            ..Default::default()
        };
        let repo = Repository::with_test_transactions(&config.database_url().as_str());

        let address = format!("http://{}/{}", config.server_address(), endpoint);
        let test_server = TestServer::new(router(config, repo)).unwrap();

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
        let body_as_str = str::from_utf8(&body).unwrap();

        let expected = serde_json::to_string(&Health {
            environment: Environment::Test,
            postgres: Pass,
        })
        .unwrap();

        assert_eq!(body_as_str, expected);
    }
}
