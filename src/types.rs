use crate::utils::contracts::IERC20::{Approval, Transfer};

pub enum DecodeResult {
    Transfer(Transfer),
    Approval(Approval),
    Unknown,
}

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

#[derive(Debug)]
pub struct TransferRow {
    pub block_timestamp: i64,
    pub block_number: i64,
    pub log_index: i64,
    pub tx_hash: String,
    pub address: String,
    pub from_address: String,
    pub to_address: String,
    pub value: String,
}

#[derive(Debug)]
pub struct ApprovalRow {
    pub block_number: i64,
    pub block_timestamp: i64,
    pub log_index: i64,
    pub tx_hash: String,
    pub address: String,
    pub owner: String,
    pub spender: String,
    pub value: String,
}

#[derive(Debug)]
pub struct TokenRow {
    pub address: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: Option<i32>,
    pub first_seen_block: i64,
    pub last_seen_block: i64,
}
