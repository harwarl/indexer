use crate::{config::Config, error::AppError};

pub mod config;
pub mod error;

#[tokio::main]
async fn main() -> Result<(), AppError>{
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().init();

    // Load application configuration from environment
    let _config = Config::from_env()?;

    
    Ok(())
}
