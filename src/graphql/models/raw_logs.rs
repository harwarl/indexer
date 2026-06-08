use async_graphql::Object;

#[derive(Debug, Clone)]
pub struct RawLog {
    pub block_number: i64,
    pub block_timestamp: i64,
    pub tx_hash: Option<String>,
    pub log_index: Option<i64>,
    pub tx_index: Option<i64>,
    pub address: String,
    pub topics: String,
    pub data: String,
}

#[Object]
impl RawLog {
    async fn block_number(&self) -> i64 {
        self.block_number
    }
    async fn block_timestamp(&self) -> i64 {
        self.block_timestamp
    }
    async fn tx_hash(&self) -> Option<&str> {
        self.tx_hash.as_deref()
    }
    async fn log_index(&self) -> Option<i64> {
        self.log_index
    }
    async fn tx_index(&self) -> Option<i64> {
        self.tx_index
    }
    async fn address(&self) -> &str {
        &self.address
    }
    async fn topics(&self) -> &str {
        &self.topics
    }
    async fn data(&self) -> &str {
        &self.data
    }
}
