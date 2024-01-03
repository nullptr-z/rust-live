use anyhow::Result;
use kv_db::{
    pb::abi::CommandRequest,
    tls::{TlsClientConnector, TlsServerAcceptor},
    ProstClientStream,
};
use tokio::net::TcpStream;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:9876";
    // tudo: 从配置文件中读取
    let ca = include_str!("../fixtures/ca.cert");

    //  create Client by connect server,
    let tcp_stream = TcpStream::connect(addr).await?;
    let tls = TlsClientConnector::new("kvserver.acme.inc", None, Some(ca))?;
    let tls_stream = tls.connect(tcp_stream).await?;
    let mut client = ProstClientStream::new(tls_stream);

    // send cmd
    let cmd = CommandRequest::new_hset("table1", "key2", "hahah");
    let data = client.execute(cmd).await?;
    info!("Got response {:?}", data);
    // let cmd = CommandRequest::new_hgetall("table1"); //, "key", "my client");
    // let data = client.execute(cmd).await?;
    // info!("Got response {:?}", data);

    Ok(())
}
