use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use venja::config::Config;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = Config::default();

    HttpServer::new(|| {
        App::new()
            // Logger middleware must be wrapped last
            .wrap(middleware::Logger::default())
    })
    .bind(config.server_address())
    .expect("Failed to bind server address")
    .run()
}
