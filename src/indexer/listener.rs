use std::sync::{Arc, Mutex};

use alloy::providers::Provider;
use futures_util::StreamExt;
use sqlx::PgPool;

use crate::{
    config::Config,
    error::AppError,
    indexer::analyzer,
    provider::connect::{connect, connect_wss},
};

pub async fn start(config: &Config, db: PgPool) -> Result<(), AppError> {
    // TODO: Backfill

    // Initialize providers
    let wss_provider = connect_wss(config.wss_rpc_url.clone()).await;
    let http_provider = Arc::new(connect(config.rpc_url.clone()).await);
    tracing::info!("Initialized Providers...");

    // 2. Subscribe to new blocks via the wss_provider
    let mut stream = wss_provider
        .subscribe_blocks()
        .await
        .map_err(|e| AppError::Provider(e.to_string()))?
        .into_stream();

    while let Some(block) = stream.next().await {
        let block_number = block.number;

        tracing::info!("Block Number: {block_number}");

        let http_provider = Arc::clone(&http_provider);
        let db = db.clone();

        tokio::spawn(async move {
            if let Err(e) = analyzer::run(http_provider, db, block_number).await {
                tracing::error!("Analyzer error on block {block_number}: {e}");
            }
        });
    }

    tracing::warn!("Block Stream Ended");

    Ok(())
}
