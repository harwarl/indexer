use axum::Error;
use thiserror::Error;

/// Errors that can occur while loading or validating application configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// One or more required environment variables were not found.
    #[error("Missing environment variables: {0}")]
    MissingVar(String),

    /// Invalid Url for one of the environmental variables
    #[error("Invalid URL format for {field}: {message}")]
    InvalidUrl { field: String, message: String },

    #[error("Invalid Chain Id")]
    InvalidChainId,
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Db Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Migration Failed: {0}")]
    MigrationFailed(String),
}

#[derive(Debug, Error)]
pub enum IndexerError{
    #[error("Failed to subscribe to blocks")]
    BlockSubscriptionFailed
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configuration Error: {0}")]
    Config(#[from] ConfigError),

    #[error("Database Connection Error: {0}")]
    Database(#[from] DatabaseError),

    #[error("Indexer Error: {0}")]
    Indexer(#[from] IndexerError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
