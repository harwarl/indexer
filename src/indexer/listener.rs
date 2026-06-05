use alloy::providers::Provider;
use sqlx::PgPool;

use crate::error::AppError;

pub async fn run<P: Provider>(wss_provider: P, http_provider: P, db: PgPool) -> Result<(), AppError>{
    // 1. Backfill from the last_block via http_provider
    // 2. Subscribe to new blocks via the wss_provider
    // 3. for each block : fetch logs - decode logs - save - checkpoint

    todo!()
}