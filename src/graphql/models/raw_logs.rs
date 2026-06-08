#[derive(Debug, Clone)]
pub struct RawLogRow {
    pub block_number: i64,
    pub block_timestamp: i64,
    pub tx_hash: Option<String>,
    pub log_index: Option<i64>,
    pub tx_index: Option<i64>,
    pub address: String,
    pub topics: String,
    pub data: String,
}

