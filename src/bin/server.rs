use dotenv::dotenv;
use venja::config::Config;
use venja::models::connection_pool;
use venja::router;

fn main() {
    dotenv().ok();
    env_logger::init();

    let config = Config::default();
    let address = config.server_address();

    let pool = connection_pool(config.database_url());

    gotham::start(address, router::router(config, pool))
}
