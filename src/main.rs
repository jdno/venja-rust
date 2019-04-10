use actix_web::{server, App, HttpRequest};
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(server_addr())
        .unwrap()
        .run();
}

fn server_addr() -> String {
    let host = env::var("VENJA_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("VENJA_PORT").unwrap_or_else(|_| "3000".to_string());

    format!("{}:{}", host, port)
}

fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

#[cfg(test)]
mod tests {
    use crate::{index, server_addr};
    use actix_web::{http, test};
    use std::env;

    #[test]
    fn index_ok() {
        let resp = test::TestRequest::with_header("content-type", "text/plain")
            .run(&index)
            .unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[test]
    fn server_addr_from_env() {
        env::set_var("VENJA_HOST", "host");
        env::set_var("VENJA_PORT", "port");

        assert_eq!(server_addr(), "host:port");
    }

    #[test]
    fn server_addr_from_defaults() {
        env::remove_var("VENJA_HOST");
        env::remove_var("VENJA_PORT");

        assert_eq!(server_addr(), "0.0.0.0:3000");
    }
}
