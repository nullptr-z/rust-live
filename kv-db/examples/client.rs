use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use kv_db::pb::abi::{CommandRequest, CommandResponse};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // 创建一个Hset命令
    let cmd = CommandRequest::new_hset("table1", "hello", "world".into());

    let addr = "127.0.0.1:9876";
    let stream = TcpStream::connect(addr).await?;

    // 使用prost协议来封包
    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();

    client.send(cmd).await?;

    if let Some(Ok(data)) = client.next().await {
        info!("Got response {:?}", data)
    }

    Ok(())
}
