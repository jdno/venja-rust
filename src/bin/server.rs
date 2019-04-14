use dotenv::dotenv;
use venja::config::Config;
use venja::router;

fn main() {
    dotenv().ok();
    env_logger::init();

    let config = Config::default();
    let address = config.server_address();

    gotham::start(address, router::router(config))
}
