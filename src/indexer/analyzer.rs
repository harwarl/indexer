use std::{str::FromStr, sync::Arc};

use alloy::{primitives::Address, providers::Provider, rpc::types::Filter};
use sqlx::PgPool;

use crate::{
    db, decoder,
    error::AppError,
    tokens::fetch::fetch_token_meta,
    types::{ApprovalRow, DecodeResult, RawLogRow, TokenRow, TransferRow},
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

        // decode the log
        match decoder::decode::try_decode(log) {
            DecodeResult::Approval(approval) => {
                approval_logs.push(ApprovalRow {
                    block_number: block_number as i64,
                    log_index: log_index.unwrap_or_default(),
                    block_timestamp: block_timestamp as i64,
                    tx_hash: tx_hash.unwrap_or_default(),
                    address: address.to_string(),
                    owner: approval.owner.to_string(),
                    spender: approval.spender.to_string(),
                    value: approval.value.to_string(),
                });
            }
            DecodeResult::Transfer(transfer) => {
                transfer_logs.push(TransferRow {
                    block_number: block_number as i64,
                    block_timestamp: block_timestamp as i64,
                    log_index: log_index.unwrap_or_default(),
                    tx_hash: tx_hash.unwrap_or_default(),
                    address: address.to_string(),
                    from_address: transfer.from.to_string(),
                    to_address: transfer.to.to_string(),
                    value: transfer.value.to_string(),
                });
            }
            DecodeResult::Unknown => {}
        };

        // inside the for log in &logs loop, after decoding:
        let token_address = address.to_string();

        if !db::tokens::token_exists(&db, &token_address).await? {
            let meta = fetch_token_meta(&http_provider, address).await;

            db::tokens::upsert_token(
                &db,
                &TokenRow {
                    address: token_address.clone(),
                    name: meta.name,
                    symbol: meta.symbol,
                    decimals: meta.decimals,
                    first_seen_block: block_number as i64,
                    last_seen_block: block_number as i64,
                },
            )
            .await?;
        } else {
            // just update last_seen_block
            db::tokens::upsert_token(
                &db,
                &TokenRow {
                    address: token_address.clone(),
                    name: None,
                    symbol: None,
                    decimals: None,
                    first_seen_block: block_number as i64,
                    last_seen_block: block_number as i64,
                },
            )
            .await?;
        }
    }

    // Save in bulk
    db::raw_logs::insert_raw_logs(&db, &raw_logs).await?;
    db::transfers::insert_transfers(&db, &transfer_logs).await?;
    db::approvals::insert_approvals(&db, &approval_logs).await?;

    tracing::info!(
        "Block {block_number}: {} logs, {} transfers, {} approvals",
        raw_logs.len(),
        transfer_logs.len(),
        approval_logs.len(),
    );

    Ok(())
}
