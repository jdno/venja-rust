//! Venja is a simple, beautifully designed habit tracker. This package
//! implements the backend of the web application, using the Actix framework and
//! GraphQL.

#![deny(missing_docs)]

#[macro_use]
extern crate diesel;
#[cfg_attr(test, macro_use)]
extern crate juniper;

pub mod config;
pub mod graphql;
pub mod handlers;
pub mod models;
pub mod repository;

/// Diesel maintains the database schema, so that the Rust compiler can check
/// the implementation against the database schema.
#[allow(missing_docs)]
pub mod schema;
