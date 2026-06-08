use async_graphql::Object;

#[derive(Debug, Clone)]
pub struct Token {
    pub address: String, 
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: Option<i32>,
    pub first_seen_block: i64,
    pub last_seen_block: i64
}

#[Object]
impl Token {
    async fn address(&self) -> &str { &self.address }
    async fn name(&self) -> Option<&str> { self.name.as_deref() }
    async fn symbol(&self) -> Option<&str> { self.symbol.as_deref() }
    async fn decimals(&self) -> Option<i32> { self.decimals }
    async fn first_seen_block(&self) -> i64 { self.first_seen_block }
    async fn last_seen_block(&self) -> i64 { self.last_seen_block }
}