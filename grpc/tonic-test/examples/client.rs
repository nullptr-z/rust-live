use std::env;

use anyhow::Result;
use social_engineer::client;
use tokio::io::{self, AsyncBufReadExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let username = env::var("USERNAME")?;
    let mut client = client::Client::new(username).await;
    client.login().await?;
    client.get_message().await?;

    let mut stdin = io::BufReader::new(io::stdin()).lines();

    while let Ok(Some(line)) = stdin.next_line().await {
        client.send_message("root", line).await?;
    }

    Ok(())
}
