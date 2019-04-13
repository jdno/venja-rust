extern crate venja_api;

use actix_web::actix::System;
use actix_web::server;
use dotenv::dotenv;
use std::env;
use venja_api::config::Config;
use venja_api::{app, models};

fn main() {
    dotenv().ok();
    env_logger::init();

    let config = Config::default();
    let system = System::new("Venja");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database = models::create_db_executor(database_url);

    let address = config.server_address();

    server::new(move || app::create_app(config.clone(), database.clone()))
        .bind(address)
        .expect("Can not bind server interface")
        .start();

    system.run();
}
