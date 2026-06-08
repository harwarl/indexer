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