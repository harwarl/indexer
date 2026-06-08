use sqlx::PgPool;

use crate::{
    error::{AppError, DatabaseError},
    types::TransferRow,
};

pub async fn insert_transfers(db: &PgPool, rows: &[TransferRow]) -> Result<(), AppError> {
    if rows.is_empty() {
        return Ok(());
    }

    let block_numbers: Vec<i64> = rows.iter().map(|r| r.block_number).collect();
    let block_timestamps: Vec<i64> = rows.iter().map(|r| r.block_timestamp).collect();
    let tx_hashes: Vec<String> = rows.iter().map(|r| r.tx_hash.clone()).collect();
    let addresses: Vec<String> = rows.iter().map(|r| r.address.clone()).collect();
    let from_addresses: Vec<String> = rows.iter().map(|r| r.from_address.clone()).collect();
    let to_addresses: Vec<String> = rows.iter().map(|r| r.to_address.clone()).collect();
    let values: Vec<String> = rows.iter().map(|r| r.value.clone()).collect();
    let log_indexes: Vec<i64> = rows.iter().map(|r| r.log_index).collect();

    sqlx::query!(
        r#"
        INSERT INTO erc20_transfers (block_number, block_timestamp, tx_hash, address, from_address, to_address, value, log_index)
        SELECT * FROM UNNEST($1::bigint[], $2::bigint[], $3::text[], $4::text[], $5::text[], $6::text[], $7::text[]::numeric[], $8::bigint[])
        ON CONFLICT DO NOTHING
        "#,
        &block_numbers,
        &block_timestamps,
        &tx_hashes,
        &addresses,
        &from_addresses,
        &to_addresses,
        &values,
        &log_indexes
    )
    .execute(db)
    .await
    .map_err(|e| DatabaseError::Error(e.to_string()))?;

    Ok(())
}
