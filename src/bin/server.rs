use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use std::sync::Arc;
use venja::config::Config;
use venja::graphql::create_schema;
use venja::handlers::{graphql, health};
use venja::repository::Repository;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = Config::default();
    let server_address = config.server_address();

    let repository = Repository::new(&config.database_url());
    let schema = Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .data(repository.clone())
            .data(schema.clone())
            // Logger middleware must be wrapped last
            .wrap(middleware::Logger::default())
            .service(web::resource("/graphql").route(web::post().to_async(graphql::graphql)))
            .service(web::resource("/playground").route(web::get().to(graphql::playground)))
            .service(web::resource("/health").route(web::get().to_async(health::health)))
    })
    .bind(server_address)
    .expect("Failed to bind server address")
    .run()
}
