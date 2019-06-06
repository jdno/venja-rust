//! Venja is a simple, beautifully designed habit tracker. This package
//! implements the backend of the web application, using the Actix framework and
//! GraphQL.

#![deny(missing_docs)]

#[macro_use]
extern crate diesel;

pub mod config;
pub mod models;

/// Diesel maintains the database schema, so that the Rust compiler can check
/// the implementation against the database schema.
#[allow(missing_docs)]
pub mod schema;
