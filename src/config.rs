//! The `config` module implements helpful structs and functions that help
//! manage and maintain the configuration of the application.

use std::env;

/// The `Config` struct defines the configurable parameters of the application.
/// It can be used to inject specific configuration into the application, e.g.
/// for testing.
///
/// When the `Config` struct is initialized with default values, it tries to get
/// its configuration from environment variables. This can be used to configure
/// the application in production environments. If no environment variables are
/// set, sensible defaults for local development are used.
pub struct Config {
    /// The IP address that the application should bind to. Defaults to 0.0.0.0.
    pub host: String,

    /// The port that the application should bind to. Defaults to 3000.
    pub port: i32,
}

impl Config {
    /// Return the address for the server.
    ///
    /// The server address is constructed from the host and port in the
    /// configuration.
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
