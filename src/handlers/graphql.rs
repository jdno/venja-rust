//! The `graphql` module implements the endpoint for GraphQL queries.

use futures::future;
use gotham::handler::HandlerFuture;
use gotham::helpers::http::response::create_response;
use gotham::state::State;
use hyper::StatusCode;

/// The GraphQL endpoint has a single handler that accepts a GraphQL query,
/// executes it, and returns the results.
pub fn execute(state: State) -> Box<HandlerFuture> {
    let response = create_response(
        &state,
        StatusCode::OK,
        mime::APPLICATION_JSON,
        String::from("GraphQL endpoint"),
    );

    let future = future::ok((state, response));

    Box::new(future)
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, Environment};
    use crate::router::{router, Repository};
    use gotham::test::{TestResponse, TestServer};
    use hyper::StatusCode;
    use std::str;

    fn post_execute(endpoint: String, body: String) -> TestResponse {
        let config = Config {
            env: Environment::Test,
            ..Default::default()
        };
        let repo = Repository::with_test_transactions(&config.database_url().as_str());

        let address = format!("http://{}/{}", config.server_address(), endpoint);
        let test_server = TestServer::new(router(config, repo)).unwrap();

        test_server
            .client()
            .post(address, body, mime::APPLICATION_JSON)
            .perform()
            .unwrap()
    }

    #[test]
    fn check_status_ok() {
        let response = post_execute(String::from("graphql"), String::new());
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn check_body() {
        let response = post_execute(String::from("graphql"), String::new());

        let body = response.read_body().unwrap();
        let body_as_str = str::from_utf8(&body).unwrap();

        let expected = String::from("GraphQL endpoint");

        assert_eq!(body_as_str, expected);
    }
}
