use crate::{
    config::Config,
    error::AppError,
    provider::connect::{ProviderType, connect},
};

pub mod config;
pub mod error;
pub mod provider;

/// Application entry point.
///
/// Initializes environment variables, sets up logging,
/// loads configuration, and starts the async runtime.
#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv::dotenv().ok();
    // Initialize Logging
    tracing_subscriber::fmt().init();

    // Load application configuration from environment
    let config = Config::from_env()?;
    tracing::info!("Loaded Env values into config...");

    // Load up database
    let _pool = config.connect_db().await?;
    tracing::info!("Postgres DB connected...");

    // Initialize providers
    let _wss_provider = connect(config.wss_rpc_url.as_str(), ProviderType::WSS).await;
    let _http_provider = connect(config.rpc_url.as_str(), ProviderType::HTTP).await;
    tracing::info!("Initialized Providers...");

    // Start indexer

    // Start GraphQl server

    // Run both concurrently
    Ok(())
}

//
// sigal::ctrl_c().await.expect()
//