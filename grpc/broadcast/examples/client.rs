use anyhow::Result;
use broadcast::client;
use std::env;
use tokio::io::{self, AsyncBufReadExt};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let username = env::var("NAME")?;
    info!("connect：{:?} ", username);
    let mut client = client::Client::new(username).await;
    client.login().await?;
    client.get_message().await?;

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    while let Ok(Some(line)) = stdin.next_line().await {
        client.send_message("root", line).await?;
    }

    Ok(())
}
