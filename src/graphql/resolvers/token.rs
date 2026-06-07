use async_graphql::Object;

#[derive(Default)]
pub struct TokenQuery;

#[Object]
impl TokenQuery {
    async fn address(&self ) -> &str {
        "Some Some"
    }
}