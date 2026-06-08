use async_graphql::{Context, Object, Result as GqlResult};
use sqlx::PgPool;

use crate::{db, graphql::models::transfer::Transfer};

#[derive(Default)]
pub struct TransferQuery;

#[Object]
impl TransferQuery {
    async fn transfers(
        &self,
        ctx: &Context<'_>,
        token_address: Option<String>,
        from_address: Option<String>,
        to_address: Option<String>,
        from_block: Option<i64>,
        to_block: Option<i64>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> GqlResult<Vec<Transfer>> {
        let pool = ctx.data::<PgPool>()?;
        let transfers = db::transfers::get_transfers(
            pool,
            token_address.as_deref(),
            from_address.as_deref(),
            to_address.as_deref(),
            from_block,
            to_block,
            limit,
            offset,
        ).await?;
        Ok(transfers)
    }

    async fn transfer(
        &self,
        ctx: &Context<'_>,
        tx_hash: String,
        log_index: i64,
    ) -> GqlResult<Option<Transfer>> {
        let pool = ctx.data::<PgPool>()?;
        let transfer = db::transfers::get_transfer(pool, &tx_hash, log_index).await?;
        Ok(transfer)
    }
}