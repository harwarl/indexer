use alloy::{providers::Provider, rpc::types::Filter};
use sqlx::PgPool;

use crate::{decoder, error::AppError, types::DecodeResult};

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

    for log in &logs {
        let topics = log.topics();
        let data = log.data();
        let address = log.address();
        let tx_hash = log.transaction_hash;
        let log_index = log.log_index;
        let tx_index = log.transaction_index;

        // TODO: decode Log
        match decoder::decode::try_decode(log) {
            DecodeResult::Approval(approval) => {
                tracing::info!(
                    "Approval: {} approved {} ({})",
                    approval.owner,
                    approval.spender,
                    approval.value
                );
                // save to DB
            }
            DecodeResult::Transfer(transfer) => {
                tracing::info!(
                    "Transfer: {} → {} ({})",
                    transfer.from,
                    transfer.to,
                    transfer.value
                );
                // save to DB
            }
            DecodeResult::Unknown => {}
        }
    }

    Ok(())
}
