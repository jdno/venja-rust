//! The `health` module implements an endpoint that can be used to monitor the
//! application and its health. The endpoint checks the app itself, as well as
//! the services it depends on, and returns a detailed status report.

use crate::config::Environment;
use crate::models::{connection, PgConn};
use crate::router::AppState;
use diesel::query_dsl::RunQueryDsl;
use diesel::sql_query;
use gotham::helpers::http::response::create_response;
use gotham::state::{FromState, State};
use hyper::{Body, Response, StatusCode};
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
pub fn check(state: State) -> (State, Response<Body>) {
    let app_state = AppState::borrow_from(&state);

    let health = Health {
        environment: app_state.config.env.clone(),
        postgres: check_postgres(connection(&state)),
    };

    let response = create_response(
        &state,
        StatusCode::OK,
        mime::APPLICATION_JSON,
        serde_json::to_string(&health).expect("Failed to serialize health"),
    );
    (state, response)
}

fn check_postgres(conn: PgConn) -> Status {
    let result = sql_query("SELECT 1").execute(&conn);

    match result {
        Ok(_) => Status::Pass,
        Err(_) => Status::Fail,
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, Environment};
    use crate::handlers::health::Health;
    use crate::handlers::health::Status::Pass;
    use crate::models::connection_pool;
    use crate::router::router;
    use gotham::test::{TestResponse, TestServer};
    use hyper::StatusCode;

    fn request_check(endpoint: String) -> TestResponse {
        let config = Config {
            env: Environment::Test,
            ..Default::default()
        };
        let pool = connection_pool(config.database_url());

        let address = format!("http://{}/{}", config.server_address(), endpoint);
        let test_server = TestServer::new(router(config, pool)).unwrap();

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
        let expected = serde_json::to_string(&Health {
            environment: Environment::Test,
            postgres: Pass,
        });

        assert_eq!(&body[..], expected.unwrap().as_bytes());
    }
}
