use crate::utils::contracts::IERC20;
use alloy::{primitives::Address, providers::Provider};

#[derive(Debug)]
pub struct TokenMeta {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: Option<i32>,
}

pub async fn fetch_token_meta<P: Provider + Clone>(provider: P, address: Address) -> TokenMeta {
    let contract = IERC20::new(address, provider);

    let name = contract
        .name()
        .call()
        .await
        .unwrap_or_else(|_| "Unknown".to_string());
    let symbol = contract
        .symbol()
        .call()
        .await
        .unwrap_or_else(|_| "UKN".to_string());
    let decimals = contract.decimals().call().await.unwrap_or(18u8) as i32;

    TokenMeta {
        name: Some(name),
        symbol: Some(symbol),
        decimals: Some(decimals),
    }
}
