use async_graphql::Object;

#[derive(Debug, Clone)]
pub struct Transfer {
    pub block_timestamp: i64,
    pub block_number: i64,
    pub tx_hash: String,
    pub address: String,
    pub from_address: String,
    pub to_address: String,
    pub value: String,
}

#[Object]
impl Transfer {
    async fn block_timestamp(&self) -> i64 { self.block_timestamp }
    async fn block_number(&self) -> i64 { self.block_number }
    async fn tx_hash(&self) -> &str { &self.tx_hash }
    async fn address(&self) -> &str { &self.address }
    async fn from_address(&self) -> &str { &self.from_address }
    async fn to_address(&self) -> &str { &self.to_address }
    async fn value(&self) -> &str { &self.value }
}