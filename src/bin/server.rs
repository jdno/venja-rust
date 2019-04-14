use dotenv::dotenv;
use std::env;
use venja::config::Config;
use venja::models::connection_pool;
use venja::router;

fn main() {
    dotenv().ok();
    env_logger::init();

    let config = Config::default();
    let address = config.server_address();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = connection_pool(database_url);

    gotham::start(address, router::router(config, pool))
}
