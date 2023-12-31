use anyhow::Result;
use kv_db::{pb::abi::CommandRequest, ProstClientStream};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    //  create Client by connect server,
    let addr = "127.0.0.1:9876";
    let stream = TcpStream::connect(addr).await?;
    let mut client = ProstClientStream::new(stream);

    // send cmd
    // let cmd = CommandRequest::new_hset("table1", "key2", "hahah");
    // let data = client.execute(cmd).await?;
    // info!("Got response {:?}", data);
    let cmd = CommandRequest::new_hgetall("table1"); //, "key", "my client");
    let data = client.execute(cmd).await?;
    info!("Got response {:?}", data);

    Ok(())
}
