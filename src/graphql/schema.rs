use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};
use sqlx::PgPool;

use crate::graphql::resolvers::{
    approvals::ApprovalQuery, basic::Query, token::TokenQuery, transfers::TransferQuery,
};

#[derive(MergedObject, Default)]
pub struct QueryRoot(Query, TokenQuery, ApprovalQuery, TransferQuery);

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema(db: PgPool) -> AppSchema {
    Schema::build(QueryRoot::default(), EmptyMutation, EmptySubscription)
        .data(db)
        .finish()
}
