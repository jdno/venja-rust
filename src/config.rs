//! The `config` module implements helpful structs and functions that help
//! manage and maintain the configuration of the application.

use std::env;
use std::str::FromStr;

/// The application can run in one of three environments: test, development, or
/// production. Depending on the environment, certain features or settings might
/// be different.
#[derive(Debug, PartialEq)]
pub enum Environment {
    /// Automated testing
    Test,
    /// Local development
    Development,
    /// Production or staging
    Production,
}

impl FromStr for Environment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sanitized = String::from(s).to_ascii_lowercase();
        let env_str = sanitized.trim();

        match env_str {
            "test" => Ok(Environment::Test),
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            _ => Err(()),
        }
    }
}

/// The `Config` struct defines the configurable parameters of the application.
/// It can be used to inject specific configuration into the application, e.g.
/// for testing.
///
/// When the `Config` struct is initialized with default values, it tries to get
/// its configuration from environment variables. This can be used to configure
/// the application in production environments. If no environment variables are
/// set, sensible defaults for local development are used.
pub struct Config {
    /// The environment the application is running in. Defaults to development.
    pub env: Environment,

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
            env: env::var("VENJA_ENV")
                .unwrap_or_else(|_| String::from("development"))
                .parse()
                .expect("Failed to parse environment variable VENJA_ENV"),
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
    use super::Environment;
    use std::str::FromStr;

    fn config() -> Config {
        Config {
            env: Environment::Test,
            host: String::from("1.2.3.4"),
            port: 80,
        }
    }

    #[test]
    fn server_address() {
        assert_eq!(config().server_address(), "1.2.3.4:80");
    }

    #[test]
    fn environment_from_lowercase() {
        assert_eq!(
            Environment::from_str("development").unwrap(),
            Environment::Development
        );
    }

    #[test]
    fn environment_from_uppercase() {
        assert_eq!(Environment::from_str("TEST").unwrap(), Environment::Test);
    }

    #[test]
    fn environment_from_trimmed() {
        assert_eq!(
            Environment::from_str(" production ").unwrap(),
            Environment::Production
        );
    }

    #[test]
    fn environment_from_error() {
        assert!(Environment::from_str("venja").is_err());
    }
}
