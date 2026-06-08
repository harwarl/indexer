use async_graphql::Object;

#[derive(Default)]
pub struct TransferQuery;

#[Object]
impl TransferQuery {
    async fn get_transfer(&self) -> &str {
        "Some Transfer"
    }
}
