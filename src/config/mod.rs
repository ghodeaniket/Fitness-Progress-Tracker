use serde::Deserialize;
use std::env;
use anyhow::Result;

/// AppConfig holds all configuration for our application
#[derive(Deserialize, Clone, Debug)]
pub struct AppConfig {
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Database connection string
    pub database_url: String,
    /// JWT secret for authentication
    pub jwt_secret: String,
    /// JWT token expiration time in seconds
    pub jwt_expiration: u64,
}

impl AppConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        // Load environment variables
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()?;
        let database_url = env::var("DATABASE_URL")?;
        let jwt_secret = env::var("JWT_SECRET")?;
        let jwt_expiration = env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "86400".to_string()) // Default to 24 hours
            .parse::<u64>()?;

        Ok(Self {
            host,
            port,
            database_url,
            jwt_secret,
            jwt_expiration,
        })
    }
    
    /// Get server address in format host:port
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
