use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use venja::config::Config;
use venja::handlers;
use venja::repository::Repository;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = Config::default();
    let server_address = config.server_address();

    let repository = Repository::new(&config.database_url());

    HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .data(repository.clone())
            // Logger middleware must be wrapped last
            .wrap(middleware::Logger::default())
            .service(web::resource("/health").route(web::get().to_async(handlers::health::health)))
    })
    .bind(server_address)
    .expect("Failed to bind server address")
    .run()
}
