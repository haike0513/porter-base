use std::str::FromStr;
pub mod blobs;
pub mod ping;

use anyhow::Result;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    ping::run().await?;
    blobs::run().await?;
    Ok(())
}
