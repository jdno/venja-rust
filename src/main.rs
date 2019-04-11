mod server;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    env_logger::init();

    server::run();
}
