use std::env;

pub struct Config {
    pub host: String,
    pub port: i32,
}

impl Config {
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: env::var("HOST").unwrap_or_else(|_| String::from("0.0.0.0")),
            port: env::var("PORT")
                .unwrap_or_else(|_| String::from("3000"))
                .parse()
                .expect("Failed to parse environment variable PORT"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    fn config() -> Config {
        Config {
            host: String::from("1.2.3.4"),
            port: 80,
        }
    }

    #[test]
    fn server_address() {
        assert_eq!(config().server_address(), "1.2.3.4:80");
    }
}
