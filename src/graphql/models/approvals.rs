use async_graphql::Object;

#[derive(Debug, Clone)]
pub struct Approval {
    pub block_number: i64,
    pub block_timestamp: i64,
    pub tx_hash: String,
    pub address: String,
    pub owner: String,
    pub spender: String,
    pub value: String,
}

#[Object]
impl Approval {
    async fn block_number(&self) -> i64 { self.block_number }
    async fn block_timestamp(&self) -> i64 { self.block_timestamp }
    async fn tx_hash(&self) -> &str { &self.tx_hash }
    async fn address(&self) -> &str { &self.address }
    async fn owner(&self) -> &str { &self.owner }
    async fn spender(&self) -> &str { &self.spender }
    async fn value(&self) -> &str { &self.value }
}