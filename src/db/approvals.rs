use crate::{
    error::{AppError, DatabaseError}, graphql::models::approvals::Approval, types::ApprovalRow
};
use sqlx::PgPool;

pub async fn insert_approvals(db: &PgPool, rows: &[ApprovalRow]) -> Result<(), AppError> {
    if rows.is_empty() {
        return Ok(());
    }

    let block_numbers: Vec<i64> = rows.iter().map(|r| r.block_number).collect();
    let block_timestamps: Vec<i64> = rows.iter().map(|r| r.block_timestamp).collect();
    let tx_hashes: Vec<String> = rows.iter().map(|r| r.tx_hash.clone()).collect();
    let addresses: Vec<String> = rows.iter().map(|r| r.address.clone()).collect();
    let owners: Vec<String> = rows.iter().map(|r| r.owner.clone()).collect();
    let spenders: Vec<String> = rows.iter().map(|r| r.spender.clone()).collect();
    let values: Vec<String> = rows.iter().map(|r| r.value.clone()).collect();
    let log_indexes: Vec<i64> = rows.iter().map(|r| r.log_index).collect();

    sqlx::query!(
        r#"
        INSERT INTO erc20_approvals (block_number, block_timestamp, tx_hash, address, owner, spender, value, log_index)
        SELECT * FROM UNNEST($1::bigint[], $2::bigint[], $3::text[], $4::text[], $5::text[], $6::text[], $7::text[]::numeric[], $8::bigint[])
        ON CONFLICT DO NOTHING
        "#,
        &block_numbers,
        &block_timestamps,
        &tx_hashes,
        &addresses,
        &owners,
        &spenders,
        &values,
        &log_indexes
    )
    .execute(db)
    .await
    .map_err(|e| DatabaseError::Error(e.to_string()))?;

    Ok(())
}

pub async fn get_approvals(
    db: &PgPool,
    token_address: Option<&str>,
    owner_address: Option<&str>,
    spender_address: Option<&str>,
    from_block: Option<i64>,
    to_block: Option<i64>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<Approval>, AppError> {
    sqlx::query_as!(
    Approval,
    r#"
    SELECT block_number, block_timestamp, tx_hash, address, owner, spender, value::text as value, log_index
    FROM erc20_approvals
    WHERE ($1::text IS NULL OR address = $1)
      AND ($2::text IS NULL OR owner = $2)
      AND ($3::text IS NULL OR spender = $3)
      AND ($4::bigint IS NULL OR block_number >= $4)
      AND ($5::bigint IS NULL OR block_number <= $5)
    ORDER BY block_number DESC
    LIMIT $6 OFFSET $7
    "#,
    token_address,
    owner_address,
    spender_address,
    from_block,
    to_block,
    limit.unwrap_or(20) as i64,
    offset.unwrap_or(0) as i64,
)
    .fetch_all(db)
    .await
    .map_err(|e| DatabaseError::Error(e.to_string()).into())
}

pub async fn get_approval(
    db: &PgPool,
    tx_hash: &str,
    log_index: i64,
) -> Result<Option<Approval>, AppError> {
    sqlx::query_as!(
        Approval,
        r#"
        SELECT block_number, block_timestamp, tx_hash, address, owner, spender, value, log_index
        FROM erc20_approvals
        WHERE tx_hash = $1 AND log_index = $2
        "#,
        tx_hash,
        log_index,
    )
    .fetch_optional(db)
    .await
    .map_err(|e| DatabaseError::Error(e.to_string()).into())
}