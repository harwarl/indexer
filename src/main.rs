use crate::{config::Config, error::AppError, graphql::app::create_app};

pub mod config;
pub mod decoder;
pub mod error;
pub mod graphql;
pub mod indexer;
pub mod provider;
pub mod types;
pub mod utils;

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
    let pool = config.connect_db().await?;
    tracing::info!("Postgres DB connected...");

    // Start indexer
    let indexer = indexer::listener::start(&config, pool.clone());
    tracing::info!("Indexer started...");

    // Start GraphQl server
    let router = create_app(pool.clone()).await;

    // Serve using axum
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000")
        .await
        .map_err(anyhow::Error::from)?;

    tracing::info!("Server listening on 0.0.0.0:4000");

    // Run concurrently
    tokio::select! {
        result = axum::serve(listener, router) => {
            drop(result);
        }
        result = indexer => {
            if let Err(e) = result {
                tracing::error!("Indexer crashed: {e}");
            }
        }
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Shutdown signal received");
        }
    }

    Ok(())
}
