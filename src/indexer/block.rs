use std::sync::Arc;

use tokio::task::JoinHandle;
use alloy::providers::Provider;
use sqlx::PgPool;

use crate::{error::{AppError, IndexerError}, indexer::listener};

pub async fn start<P>( wss_provider: P, http_provider: P, db: PgPool) 
-> Result<JoinHandle<()>, AppError> 
where P: Provider + 'static
{
    // Get the last block via checkpoint
    let wss_provider = Arc::new(wss_provider);
    let http_provider = Arc::new(http_provider);

    // Split into threads to handle the blocks
    let handle = tokio::spawn(async move {
        // Clone the providers
        let wss_provider = Arc::clone(&wss_provider);
        let http_provider = Arc::clone(&http_provider);

        if let Err(e) = listener::run(wss_provider, http_provider, db.clone()).await {
            tracing::error!("Indexer Error: {e}")
        }
    });

    Ok(handle)
}