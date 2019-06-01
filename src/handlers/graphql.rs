//! The `graphql` module implements the endpoint for GraphQL queries.

use crate::router::{AppState, Repository};
use futures::future;
use futures::future::Future;
use futures::stream::Stream;
use gotham::handler::{HandlerFuture, IntoHandlerError};
use gotham::helpers::http::response::create_response;
use gotham::router::response::extender::StaticResponseExtender;
use gotham::state::{FromState, State, StateData};
use hyper::{Body, Response, StatusCode};
use juniper::http::GraphQLRequest;
use juniper::{DefaultScalarValue, InputValue, ScalarValue};
use serde::{Deserialize, Serialize};

/// The GraphQL extractor is used to parse an incoming GraphQL request, and
/// extract its different parts into the following struct.
#[derive(Serialize, Deserialize)]
pub struct GraphQLRequestExtractor<S = DefaultScalarValue>
where
    S: ScalarValue + Sync + Send,
{
    query: String,
    #[serde(rename = "operationName")]
    operation_name: Option<String>,
    #[serde(bound(deserialize = "InputValue<S>: Deserialize<'de> + Serialize"))]
    variables: Option<InputValue<S>>,
}

impl StateData for GraphQLRequestExtractor {}
impl StaticResponseExtender for GraphQLRequestExtractor {
    type ResBody = Body;
    fn extend(_state: &mut State, _res: &mut Response<Body>) {}
}

/// Respond to a GraphQL query via POST.
pub fn post(mut state: State) -> Box<HandlerFuture> {
    let future = Body::take_from(&mut state)
        .concat2()
        .then(|body| match body {
            Ok(body) => match String::from_utf8(body.to_vec()) {
                Ok(json) => match serde_json::from_str(json.as_str()) {
                    Ok(request) => future::ok(execute(state, request)),
                    Err(e) => future::err((state, e.into_handler_error())),
                },
                Err(e) => future::err((state, e.into_handler_error())),
            },
            Err(e) => future::err((state, e.into_handler_error())),
        });

    Box::new(future)
}

fn execute(state: State, graphql: GraphQLRequest) -> (State, Response<Body>) {
    let graphql_schema = AppState::borrow_from(&state).schema.clone();
    let repository = Repository::borrow_from(&state).clone();

    let result = graphql.execute(&graphql_schema, &repository);

    let status_code = if result.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    };
    let response_as_json = serde_json::to_string_pretty(&result).unwrap_or_else(|_| String::new());

    let response = create_response(
        &state,
        status_code,
        mime::APPLICATION_JSON,
        response_as_json,
    );

    (state, response)
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
    fn execute_status_ok() {
        let response = post_execute(
            String::from("graphql"),
            String::from("{\"query\":\"{ apiVersion }\"}"),
        );
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    #[ignore]
    fn execute_status_bad_request() {
        let response = post_execute(String::from("graphql"), String::from("{}"));
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn execute_body_ok() {
        let response = post_execute(
            String::from("graphql"),
            String::from("{\"query\":\"{ apiVersion }\"}"),
        );

        let body = response.read_body().unwrap();
        let body_as_str = str::from_utf8(&body).unwrap();

        let expected =
            serde_json::to_string_pretty(&json!({ "data": { "apiVersion": "1.0" }})).unwrap();

        assert_eq!(body_as_str, expected);
    }
}
