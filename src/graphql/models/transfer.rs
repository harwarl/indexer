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

