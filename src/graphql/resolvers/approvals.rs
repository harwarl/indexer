use async_graphql::Object;

#[derive(Default)]
pub struct ApprovalQuery;


#[Object]
impl ApprovalQuery {
    async fn get_approval(&self) -> &str{
        "Some Approval"
    }
}