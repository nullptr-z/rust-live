use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use kv_server::{CommandRequest, CommandResponse, PostClientStream};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:7879";
    let stream = TcpStream::connect(addr).await?;
    let mut client = PostClientStream::new(stream);

    // 创建一个 hset 命令
    let cmd = CommandRequest::new_hset("table1", "my_key", "world".into());
    let res = client.execute(cmd).await?;
    println!("【 res 】==> {:?}", res);
    // let cmd = CommandRequest::new_hset("table1", "kv", "kv server".into());
    // client.execute(cmd).await?;
    // let cmd = CommandRequest::new_hget("table1", "my_key");
    // client.execute(cmd).await?;
    // let cmd = CommandRequest::new_hgetall("table1");
    // client.execute(cmd).await?;

    Ok(())
}
