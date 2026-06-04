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
}


#[derive(Debug, Error)]
pub enum AppError {
    #[error("Configguration Error: {0}")]
    Config(#[from] ConfigError)
}