use dotenv::dotenv;
use venja::config::Config;
use venja::router;
use venja::router::Repository;

fn main() {
    dotenv().ok();
    env_logger::init();

    let config = Config::default();
    let address = config.server_address();

    let repo = Repository::new(&config.database_url().as_str());

    gotham::start(address, router::router(config, repo))
}
