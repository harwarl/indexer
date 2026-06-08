use async_graphql::Object;

#[derive(Default)]
pub struct TokenQuery;

#[Object]
impl TokenQuery {
    async fn tokens(&self) -> &str {
        todo!()
    }

    async fn token(&self) -> &str {
        todo!()
    }
}
