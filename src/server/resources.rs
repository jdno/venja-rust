use actix_web::HttpRequest;

pub fn index(_req: &HttpRequest) -> &'static str {
    "Hello world!"
}

#[cfg(test)]
mod tests {
    use crate::server::resources::index;
    use actix_web::{http, test};

    #[test]
    fn index_ok() {
        let resp = test::TestRequest::with_header("content-type", "text/plain")
            .run(&index)
            .unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
