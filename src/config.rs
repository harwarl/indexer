use std::env;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use url::Url;

use crate::error::{ConfigError, DatabaseError};

/// Application configuration loaded from environment variables or other sources.
#[derive(Debug)]
pub struct Config {
    pub rpc_url: String,
    pub wss_rpc_url: String,
    pub database_url: String,
    pub chain_id: u32,
}

impl Config {
    /// Creates a new configuration instance using the default configuration source.
    ///
    /// # Returns
    ///
    /// A fully initialized [`Config`] instance.
    pub fn new(
        rpc_url: String,
        wss_rpc_url: String,
        database_url: String,
        chain_id: u32,
    ) -> Result<Self, ConfigError> {
        // Validate the urls
        validate_url(&rpc_url, "RPC_URL", &["https", "http"])?;
        validate_url(&wss_rpc_url, "RPC_URL_WSS", &["wss", "ws"])?;
        validate_url(&database_url, "DATABASE_URL", &["postgres", "postgresql"])?;

        Ok(Self {
            rpc_url,
            wss_rpc_url,
            database_url,
            chain_id,
        })
    }

    /// Loads configuration values from environment variables.
    ///
    /// # Returns
    ///
    /// A [`Config`] instance populated with values read from the current
    /// process environment.
    ///
    /// # Panics
    ///
    /// May panic if required environment variables are missing or invalid,
    /// depending on the implementation.
    pub fn from_env() -> Result<Self, ConfigError> {
        // Get all the required keys and panic if any fails
        let rpc_url = env::var("RPC_URL")
            .map_err(|_| ConfigError::MissingVar("RPC_URL is missing".to_string()))?;

        let rpc_url_wss = env::var("RPC_URL_WSS")
            .map_err(|_| ConfigError::MissingVar("RPC_URL_WSS is missing".to_string()))?;

        let database_url =
            env::var("DATABASE_URL").unwrap_or("https://Someurl.com/indexer".to_string());

        let chain_id: u32 = env::var("CHAIN_ID")
            .unwrap_or_else(|_| "11155111".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidChainId)?;

        Self::new(rpc_url, rpc_url_wss, database_url, chain_id)
    }

    /// Establishes a connection pool to the PostgreSQL database and runs all pending migrations.
    ///
    /// # Errors
    ///
    /// Returns [`DatabaseError::ConnectionFailed`] if the pool cannot connect to the database,
    /// or [`DatabaseError::MigrationFailed`] if any migration fails to apply.
    pub async fn connect_db(self: &Self) -> Result<Pool<Postgres>, DatabaseError> {
        let pool = PgPoolOptions::new()
            .connect(&self.database_url)
            .await
            .map_err(|e| DatabaseError::ConnectionFailed(e.to_string()))?;
        sqlx::migrate!("db/migrations")
            .run(&pool)
            .await
            .map_err(|e| DatabaseError::MigrationFailed(e.to_string()))?;
        Ok(pool)
    }
}

/// Validates that a URL is properly formatted and uses http/https only.
///
/// # Errors
/// Returns `ConfigError::InvalidUrl` if parsing fails or scheme is unsupported.
fn validate_url(url: &str, field: &str, allowed: &[&str]) -> Result<(), ConfigError> {
    let parsed = Url::parse(url).map_err(|e| ConfigError::InvalidUrl {
        field: field.to_string(),
        message: e.to_string(),
    })?;

    let scheme = parsed.scheme();

    if allowed.contains(&scheme) {
        Ok(())
    } else {
        return Err(ConfigError::InvalidUrl {
            field: field.to_string(),
            message: format!("Expected http/https, got {scheme}"),
        });
    }
}
