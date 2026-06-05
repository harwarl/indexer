use alloy::providers::Provider;
use sqlx::PgPool;

use crate::error::{AppError, IndexerError};

pub async fn run<P>(http_provider: P, db: PgPool) -> Result<(), AppError>
where
    P: Provider,
{
    // Analyze the current block
    Ok(())
}
