//! The `health` module implements an endpoint that can be used to monitor the
//! application and its health. The endpoint monitors the app itself, as well as
//! the services it depends on, and returns a detailed status report.

use crate::config::{Config, Environment};
use crate::repository::Repository;
use actix_web::error::BlockingError;
use actix_web::{web, Error, HttpResponse};
use diesel::query_dsl::RunQueryDsl;
use diesel::sql_query;
use futures::Future;
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
pub fn health(
    config: web::Data<Config>,
    repository: web::Data<Repository>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || Ok(check(config, repository))).then(
        |result: Result<Health, BlockingError<()>>| match result {
            Ok(health) => Ok(HttpResponse::Ok().json(health)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        },
    )
}

fn check(config: web::Data<Config>, repository: web::Data<Repository>) -> Health {
    match repository.connection() {
        Ok(connection) => {
            let query_result = sql_query("SELECT 1").execute(&connection);

            let postgres_status = match query_result {
                Ok(_) => Status::Pass,
                Err(_) => Status::Fail,
            };

            Health {
                environment: config.env.clone(),
                postgres: postgres_status,
            }
        }
        Err(_) => Health {
            environment: config.env.clone(),
            postgres: Status::Fail,
        },
    }
}
