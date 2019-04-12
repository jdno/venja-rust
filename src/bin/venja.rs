extern crate venja_api;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    env_logger::init();
}
