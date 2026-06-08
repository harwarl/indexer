use async_graphql::{Context, Object};

#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> &str {
        "Hello World"
    }
}
