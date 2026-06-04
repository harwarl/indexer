use alloy::providers::{Provider, ProviderBuilder, WsConnect};

#[derive(Debug)]
pub enum ProviderType {
    HTTP,
    WSS,
}

/// Creates a provider connection for either HTTP or WSS
///
/// # Arguments
/// * `url` - A HTTP or Websocket URL e.g. `wss://mainnet.infura.io/ws/v3/<key>` or `https://mainnet.infura.io/v3/<key>`
///
/// # Panics
/// Panics if the connection cannot be established
pub async fn connect(url: &str, provider_type: ProviderType) -> impl Provider {
    match provider_type {
        ProviderType::HTTP => ProviderBuilder::new()
            .connect(url)
            .await
            .expect("Failed to connect to provider"),
        ProviderType::WSS => ProviderBuilder::new()
            .connect_ws(WsConnect::new(url))
            .await
            .expect("Failed to connect to websocket"),
    }
}
