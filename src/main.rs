mod server;

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    server::run();
}
