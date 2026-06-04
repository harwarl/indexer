use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use sqlx::PgPool;

pub struct Query;

#[Object]
impl Query {
    async fn hello(self: &Self) -> String {
        "Hello World".to_string()
    }
}

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn build_schema(db: PgPool) -> AppSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(db)
        .finish()
}
