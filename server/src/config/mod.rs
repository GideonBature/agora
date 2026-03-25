use std::env;

pub mod cors;
pub mod request_id;
pub mod security;

pub use cors::create_cors_layer;
pub use request_id::{propagate_request_id_layer, set_request_id_layer};
pub use security::create_security_headers_layer;

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct Config {
    /// Database connection URL.
    pub database_url: String,

    /// Server port (default: 3001).
    pub port: u16,

    /// Environment (development, production, testing).
    pub rust_env: String,

    /// Comma-separated list of allowed origins for CORS.
    pub cors_allowed_origins: String,

    /// Logging configuration (RUST_LOG).
    pub rust_log: String,
}

impl Config {
    /// Load configuration from environment variables with sensible defaults.
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/agora".to_string()),

            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3001),

            rust_env: env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()),

            cors_allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".to_string()),

            rust_log: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        }
    }

    /// Helper to identify if running in production.
    pub fn is_production(&self) -> bool {
        self.rust_env.to_lowercase() == "production"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use temp_env;

    #[test]
    fn test_config_from_env_defaults() {
        // Ensure that clearing environment variables doesn't break initialization.
        // In practice we can't easily clear all global env in parallel tests,
        // but we can verify that the default values are correct if variables are unset.

        // We'll just test that from_env() at least works and has expected structure.
        let config = Config::from_env();
        assert!(config.port > 0);
    }

    #[test]
    fn test_is_production() {
        let mut config = Config::from_env();
        config.rust_env = "production".into();
        assert!(config.is_production());

        config.rust_env = "development".into();
        assert!(!config.is_production());
    }

    #[tokio::test]
    async fn test_port_from_env_variable() {
        // Test that PORT environment variable is correctly read
        temp_env::async_with_vars([("PORT", Some("8080"))], async {
            let config = Config::from_env();
            assert_eq!(config.port, 8080);
        })
        .await;
    }

    #[tokio::test]
    async fn test_port_default_when_not_set() {
        // Test that default port 3001 is used when PORT is not set
        temp_env::async_with_vars([("PORT", None::<&str>)], async {
            let config = Config::from_env();
            assert_eq!(config.port, 3001);
        })
        .await;
    }

    #[tokio::test]
    async fn test_port_invalid_value_falls_back_to_default() {
        // Test that invalid port values fall back to default
        temp_env::async_with_vars([("PORT", Some("invalid"))], async {
            let config = Config::from_env();
            assert_eq!(config.port, 3001);
        })
        .await;
    }

    #[tokio::test]
    async fn test_port_valid_range_values() {
        // Test various valid port values
        let valid_ports = [80, 443, 8000, 8080, 9000, 65535];

        for port in valid_ports {
            temp_env::async_with_vars([("PORT", Some(port.to_string()))], async {
                let config = Config::from_env();
                assert_eq!(config.port, port);
            })
            .await;
        }
    }
}
