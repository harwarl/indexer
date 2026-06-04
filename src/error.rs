use thiserror::Error;

/// Errors that can occur while loading or validating application configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// One or more required environment variables were not found.
    #[error("Missing environment variables: {0}")]
    MissingVar(String),
}
