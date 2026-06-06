use sqlx::PgPool;

use crate::{
    error::{AppError, DatabaseError},
    types::RawLogRow,
};

// This contains all DB operations related to raw logs
pub async fn insert_raw_logs(db: &PgPool, rows: &[RawLogRow]) -> Result<(), AppError> {
    if rows.is_empty() {
        return Ok(());
    }

    let block_numbers: Vec<i64> = rows.iter().map(|r| r.block_number).collect();
    let block_timestamps: Vec<i64> = rows.iter().map(|r| r.block_timestamp).collect();
    let tx_hashes: Vec<Option<String>> = rows.iter().map(|r| r.tx_hash.clone()).collect();
    let tx_indexes: Vec<Option<i64>> = rows.iter().map(|r| r.tx_index).collect();
    let log_indexes: Vec<Option<i64>> = rows.iter().map(|r| r.log_index).collect();
    let addresses: Vec<String> = rows.iter().map(|r| r.address.clone()).collect();
    let data: Vec<String> = rows.iter().map(|r| r.data.clone()).collect();

    sqlx::query!(
        r#"
        INSERT INTO raw_logs (block_number, block_timestamp, tx_hash, tx_index, log_index, address, data)
        SELECT * FROM UNNEST($1::bigint[], $2::bigint[], $3::text[], $4::bigint[], $5::bigint[], $6::text[], $7::text[])
        ON CONFLICT (tx_hash, log_index) DO NOTHING
        "#,
        &block_numbers,
        &block_timestamps,
        &tx_hashes as &[Option<String>],
        &tx_indexes as &[Option<i64>],
        &log_indexes as &[Option<i64>],
        &addresses,
        &data,
    )
    .execute(db)
    .await
    .map_err(|e| DatabaseError::Error(e.to_string()))?;

    Ok(())
}
