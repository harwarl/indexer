use crate::{
    error::{AppError, DatabaseError},
    types::TokenRow,
};
use sqlx::PgPool;

pub async fn upsert_token(db: &PgPool, token: &TokenRow) -> Result<(), AppError> {
    sqlx::query!(
        r#"
        INSERT INTO tokens (address, name, symbol, decimals, first_seen_block, last_seen_block)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (address) DO UPDATE SET
            last_seen_block = GREATEST(tokens.last_seen_block, EXCLUDED.last_seen_block),
            name    = COALESCE(tokens.name, EXCLUDED.name),
            symbol  = COALESCE(tokens.symbol, EXCLUDED.symbol),
            decimals = COALESCE(tokens.decimals, EXCLUDED.decimals)
        "#,
        token.address,
        token.name,
        token.symbol,
        token.decimals,
        token.first_seen_block,
        token.last_seen_block,
    )
    .execute(db)
    .await
    .map_err(|e| DatabaseError::Error(e.to_string()))?;

    Ok(())
}

pub async fn token_exists(db: &PgPool, address: &str) -> Result<bool, AppError> {
    let result = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM tokens WHERE address = $1)",
        address
    )
    .fetch_one(db)
    .await
    .map_err(|e| DatabaseError::Error(e.to_string()))?;

    Ok(result.unwrap_or(false))
}
