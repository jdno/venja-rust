//! The `graphql` module implements two endpoints: one for queries itself, and
//! one for a graphical interface that can be used to develop and test the
//! service.

use crate::graphql::Schema;
use crate::repository::Repository;
use actix_web::{web, Error, HttpResponse};
use futures::Future;
use juniper::http::playground::playground_source;
use juniper::http::GraphQLRequest;
use std::sync::Arc;

/// The GraphQL endpoint executes a given GraphQL query and returns its result.
pub fn graphql(
    repository: web::Data<Repository>,
    schema: web::Data<Arc<Schema>>,
    query: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let result = query.execute(&schema, &repository);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&result)?)
    })
    .map_err(Error::from)
    .and_then(|result| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(result))
    })
}

/// The GraphQL Playground is a web-based client that can be used to interact
/// with the GraphQL endpoint.
pub fn playground() -> HttpResponse {
    let html = playground_source("http://127.0.0.1:3000/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
