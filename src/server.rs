mod resources;

use self::resources::index;
use actix_web::{server, App};
use std::env;

pub fn run() {
    server::new(|| App::new().resource("/", |r| r.f(index)))
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
    use std::env;

    #[test]
    fn server_addr_from_env() {
        env::set_var("VENJA_HOST", "host");
        env::set_var("VENJA_PORT", "port");

        assert_eq!(addr(), "host:port");
    }

    #[test]
    fn server_addr_from_defaults() {
        env::remove_var("VENJA_HOST");
        env::remove_var("VENJA_PORT");

        assert_eq!(addr(), "0.0.0.0:3000");
    }
}
