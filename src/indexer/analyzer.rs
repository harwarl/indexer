use alloy::{
    network::TransactionResponse,
    providers::Provider,
    rpc::types::{Filter, TransactionReceipt},
};
use sqlx::PgPool;

use crate::error::AppError;

pub async fn run<P>(http_provider: P, db: PgPool, block_number: u64) -> Result<(), AppError>
where
    P: Provider,
{
    // Analyze the current block
    // Get the block details from the http provider
    let block = http_provider
        .get_block_by_number(block_number.into())
        .hashes()
        .await
        .map_err(|e| AppError::Provider(e.to_string()))?
        .ok_or_else(|| AppError::Provider("Block not found".to_string()))?;

    let filter = Filter::new()
        .from_block(block_number)
        .to_block(block_number);

    let logs = http_provider
        .get_logs(&filter)
        .await
        .map_err(|e| AppError::Provider(e.to_string()))?;

    let block_timestamp = block.header.timestamp;

    for log in logs {
        let topics = log.topics();
        let data = log.data();
        let address = log.address();
        let tx_hash = log.transaction_hash;
        let log_index = log.log_index;
        let tx_index = log.transaction_index;

        // TODO: decode Log
        // TODO: save to DB
    }

    Ok(())
}
