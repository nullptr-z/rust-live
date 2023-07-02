use std::time::Duration;

use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use kv_server::{CommandRequest, CommandResponse};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:7878";
    let stream = TcpStream::connect(addr).await?;
    // 将tcp stream帧封装成 protobuf 消息
    let mut client =
        AsyncProstStream::<_, CommandResponse, CommandRequest, _>::from(stream).for_async();

    // 创建一个 hset 命令
    let cmd = CommandRequest::new_hset("table1", "my_key", "world".into());
    client.send(cmd).await?;
    client.next().await;

    let cmd = CommandRequest::new_hset("table1", "kv", "kv server".into());
    client.send(cmd).await?;
    client.next().await;

    // let data = client.next().await;
    // info!("Got response: {:#?}", data);

    // std::thread::sleep(Duration::from_secs(2));
    let cmd = CommandRequest::new_hget("table1", "my_key");
    client.send(cmd).await?;
    client.next().await;

    let cmd = CommandRequest::new_hgetall("table1");
    client.send(cmd).await?;
    let data = client.next().await;
    info!("Got response: {:#?}", data);

    // while let Some(data) = client.next().await {
    //     info!("Got response: {:#?}", data);
    // }

    Ok(())
}
