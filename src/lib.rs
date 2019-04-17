//! Venja is a simple, beautifully designed habit tracker. This package
//! implements the backend of the web application, using the Actix framework and
//! GraphQL.

#![deny(missing_docs)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate gotham_derive;
#[macro_use]
extern crate log;

pub mod config;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod router;

/// Diesel maintains the database schema, so that the Rust compiler can check
/// the implementation against the database schema.
#[allow(missing_docs)]
pub mod schema;
