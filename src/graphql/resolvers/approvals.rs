use async_graphql::{Context, Object, Result as GqlResult};
use sqlx::PgPool;

use crate::{db::approvals::{get_approval, get_approvals}, graphql::models::approvals::Approval};

#[derive(Default)]
pub struct ApprovalQuery;

#[Object]
impl ApprovalQuery {
    async fn approvals<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        token_address: Option<String>,
        owner_address: Option<String>,
        spender_address: Option<String>,
        from_block: Option<i64>,
        to_block: Option<i64>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> GqlResult<Vec<Approval>> {
        let pool = ctx.data::<PgPool>()?;
        let approvals = get_approvals(
            pool,
            token_address.as_deref(),
            owner_address.as_deref(),
            spender_address.as_deref(),
            from_block,
            to_block,
            limit,
            offset,
        )
        .await?;
        Ok(approvals)
    }

    async fn approval<'ctx>(&self, ctx: &Context<'ctx>, tx_hash: String, log_index: i64) -> GqlResult<Option<Approval>> {
        let pool = ctx.data::<PgPool>()?;
        let approval = get_approval(pool, &tx_hash, log_index).await?;
        Ok(approval)
    }
}
