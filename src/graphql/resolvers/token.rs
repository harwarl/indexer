use async_graphql::{Context, Object, Result as GqlResult};
use sqlx::PgPool;

use crate::{db::tokens::{get_token, get_tokens}, error::AppError, graphql::models::token::Token};

#[derive(Default)]
pub struct TokenQuery;

#[Object]
impl TokenQuery {
    async fn tokens<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> GqlResult<Vec<Token>> {
        let pool = ctx.data::<PgPool>()?;
        let tokens = get_tokens(pool, limit, offset).await?;
        Ok(tokens)
    }

    async fn token<'ctx>(&self, ctx: &Context<'ctx>, address: String) -> GqlResult<Option<Token>> {
        let pool = ctx.data::<PgPool>()?;
        let token = get_token(pool, address.as_str()).await?;
        Ok(token)
    }
}
