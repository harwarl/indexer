use alloy::{providers::Provider, rpc::types::Filter};
use sqlx::PgPool;

use crate::{
    decoder,
    error::AppError,
    types::{ApprovalRow, DecodeResult, RawLogRow, TransferRow},
};

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

    // collect into vecs before touching DB
    let mut raw_logs: Vec<RawLogRow> = Vec::new();
    let mut transfer_logs: Vec<TransferRow> = Vec::new();
    let mut approval_logs: Vec<ApprovalRow> = Vec::new();

    for log in &logs {
        let topics = log.topics().iter().map(|t| t.to_string()).collect();
        let data = log.data().data.to_string();
        let address = log.address();
        let tx_hash = log.transaction_hash.map(|h| h.to_string());
        let log_index = log.log_index.map(|i| i as i64);
        let tx_index = log.transaction_index.map(|i| i as i64);

        raw_logs.push(RawLogRow {
            block_number: block_number as i64,
            block_timestamp: block_timestamp as i64,
            tx_hash: tx_hash.clone(),
            log_index,
            tx_index,
            address: address.to_string().clone(),
            topics,
            data,
        });

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
                approval_logs.push(ApprovalRow {
                    block_number: block_number as i64,
                    block_timestamp: block_timestamp as i64,
                    tx_hash: tx_hash.unwrap_or_default(),
                    address: address.to_string(),
                    owner: approval.owner.to_string(),
                    spender: approval.spender.to_string(),
                    value: approval.value.to_string(),
                });
            }
            DecodeResult::Transfer(transfer) => {
                tracing::info!(
                    "Transfer: {} → {} ({})",
                    transfer.from,
                    transfer.to,
                    transfer.value
                );
                // save to DB
                transfer_logs.push(TransferRow {
                    block_number: block_number as i64,
                    block_timestamp: block_timestamp as i64,
                    tx_hash: tx_hash.unwrap_or_default(),
                    address: address.to_string(),
                    from_address: transfer.from.to_string(),
                    to_address: transfer.to.to_string(),
                    value: transfer.value.to_string(),
                });
            }
            DecodeResult::Unknown => {}
        }
    }

    // TODO: Save in Batches

    tracing::info!(
        "Block {block_number}: {} logs, {} transfers, {} approvals",
        raw_logs.len(),
        transfer_logs.len(),
        approval_logs.len(),
    );

    Ok(())
}
