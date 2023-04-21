use anyhow::Result;
use std::env;
use tokio::io::{self, AsyncBufReadExt};
use tonic_live::client::Client;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let username = env::var("USERNAME").unwrap();
    println!("【 username 】==> {:?}", username);
    let mut client = Client::new(username).await;
    client.login().await?;
    client.get_messages().await?;

    // 从标准输入读取消息并发送给服务器
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    while let Ok(Some(line)) = stdin.next_line().await {
        client.send_message("lobby", line).await?;
    }
    Ok(())
}
