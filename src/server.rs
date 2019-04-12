mod resources;

use self::resources::index;
use actix_web::middleware::Logger;
use actix_web::{server, App};
use std::env;

pub fn run() {
    server::new(|| {
        App::new()
            .middleware(Logger::default())
            .resource("/", |r| r.f(index))
    })
    .bind(addr())
    .unwrap()
    .run();
}

fn addr() -> String {
    let host = env::var("VENJA_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("VENJA_PORT").unwrap_or_else(|_| "3000".to_string());

    format!("{}:{}", host, port)
}

#[cfg(test)]
mod tests {
    use crate::server::addr;

    #[test]
    fn server_addr_from_defaults() {
        assert_eq!(addr(), "0.0.0.0:3000");
    }
}
