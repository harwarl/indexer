use sqlx::PgPool;

use crate::{
    error::{AppError, DatabaseError},
    graphql::models::{raw_logs::RawLog},
    types::RawLogRow,
};

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
    let topics: Vec<String> = rows.iter().map(|r| r.topics.clone()).collect();
    let data: Vec<String> = rows.iter().map(|r| r.data.clone()).collect();

    sqlx::query!(
        r#"
        INSERT INTO raw_logs (block_number, block_timestamp, tx_hash, tx_index, log_index, address, topics, data)
        SELECT * FROM UNNEST($1::bigint[], $2::bigint[], $3::text[], $4::bigint[], $5::bigint[], $6::text[], $7::text[], $8::text[])
        ON CONFLICT (tx_hash, log_index) DO NOTHING
        "#,
        &block_numbers,
        &block_timestamps,
        &tx_hashes as &[Option<String>],
        &tx_indexes as &[Option<i64>],
        &log_indexes as &[Option<i64>],
        &addresses,
        &topics,
        &data,
    )
    .execute(db)
    .await
    .map_err(|e| DatabaseError::Error(e.to_string()))?;

    Ok(())
}

pub async fn get_raw_logs(
    db: &PgPool,
    address: Option<&str>,
    tx_hash: Option<&str>,
    from_block: Option<i64>,
    to_block: Option<i64>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<RawLog>, AppError> {
    sqlx::query_as!(
        RawLog,
        r#"
        SELECT block_number, block_timestamp, tx_hash, tx_index, log_index, address, topics, data
        FROM raw_logs
        WHERE ($1::text IS NULL OR address = $1)
          AND ($2::text IS NULL OR tx_hash = $2)
          AND ($3::bigint IS NULL OR block_number >= $3)
          AND ($4::bigint IS NULL OR block_number <= $4)
        ORDER BY block_number DESC
        LIMIT $5 OFFSET $6
        "#,
        address,
        tx_hash,
        from_block,
        to_block,
        limit.unwrap_or(20) as i64,
        offset.unwrap_or(0) as i64,
    )
    .fetch_all(db)
    .await
    .map_err(|e| DatabaseError::Error(e.to_string()).into())
}

pub async fn get_raw_log(
    db: &PgPool,
    tx_hash: &str,
    log_index: i64,
) -> Result<Option<RawLog>, AppError> {
    sqlx::query_as!(
        RawLog,
        r#"
        SELECT block_number, block_timestamp, tx_hash, tx_index, log_index, address, topics, data
        FROM raw_logs
        WHERE tx_hash = $1 AND log_index = $2
        "#,
        tx_hash,
        log_index,
    )
    .fetch_optional(db)
    .await
    .map_err(|e| DatabaseError::Error(e.to_string()).into())
}