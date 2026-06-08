use crate::{
    error::{AppError, DatabaseError},
    graphql::models::token::Token,
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

pub async fn get_token(db: &PgPool, address: &str) -> Result<Option<Token>, AppError> {
    sqlx::query_as!(
        Token,
        "SELECT address, name, symbol, decimals, first_seen_block, last_seen_block FROM tokens WHERE address = $1",
        address
    )
    .fetch_optional(db)
    .await
    .map_err(|e| DatabaseError::Error(e.to_string()).into())
}

pub async fn get_tokens(
    db: &PgPool,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<Vec<Token>, AppError> {
    sqlx::query_as!(
        Token,
        "SELECT address, name, symbol, decimals, first_seen_block, last_seen_block FROM tokens ORDER BY last_seen_block DESC LIMIT $1 OFFSET $2",
        limit.unwrap_or(20) as i64,
        offset.unwrap_or(0) as i64,
    )
    .fetch_all(db)
    .await
    .map_err(|e| DatabaseError::Error(e.to_string()).into())
}
