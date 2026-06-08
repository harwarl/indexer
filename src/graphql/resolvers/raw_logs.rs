use async_graphql::{Context, Object, Result as GqlResult};
use sqlx::PgPool;

use crate::{db, graphql::models::raw_logs::RawLog};

#[derive(Default)]
pub struct RawLogsQuery;

#[Object]
impl RawLogsQuery {
    async fn logs(
        &self,
        ctx: &Context<'_>,
        address: Option<String>,
        tx_hash: Option<String>,
        from_block: Option<i64>,
        to_block: Option<i64>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> GqlResult<Vec<RawLog>> {
        let pool = ctx.data::<PgPool>()?;
        let logs = db::raw_logs::get_raw_logs(
            pool,
            address.as_deref(),
            tx_hash.as_deref(),
            from_block,
            to_block,
            limit,
            offset,
        ).await?;
        Ok(logs)
    }

    async fn log(
        &self,
        ctx: &Context<'_>,
        tx_hash: String,
        log_index: i64,
    ) -> GqlResult<Option<RawLog>> {
        let pool = ctx.data::<PgPool>()?;
        let log = db::raw_logs::get_raw_log(pool, &tx_hash, log_index).await?;
        Ok(log)
    }
}