use crate::{
    error::{AppError, DatabaseError},
    types::ApprovalRow,
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

    sqlx::query!(
        r#"
        INSERT INTO erc20_approvals (block_number, block_timestamp, tx_hash, address, owner, spender, value)
        SELECT * FROM UNNEST($1::bigint[], $2::bigint[], $3::text[], $4::text[], $5::text[], $6::text[], $7::text[]::numeric[])
        ON CONFLICT DO NOTHING
        "#,
        &block_numbers,
        &block_timestamps,
        &tx_hashes,
        &addresses,
        &owners,
        &spenders,
        &values,
    )
    .execute(db)
    .await
    .map_err(|e| DatabaseError::Error(e.to_string()))?;

    Ok(())
}
