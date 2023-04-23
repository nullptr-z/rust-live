use anyhow::{anyhow, Result};
use s2n_quic::{client::Connect, Client};
use std::net::SocketAddr;
use tokio::io;

static LOCALHOST: &str = "127.0.0.1";

const CERT_PEM: &str = include_str!("../fixtures/cert.pem");
// const KEY_PEM: &str = include_str!("../fixtures/key.pem");

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        // 提供 server 的证书
        .with_tls(CERT_PEM)?
        // 这里并不使用传统的bind函数，而是with_io
        // 0.0.0.0:0 表示随机分配一个端口;
        .with_io("0.0.0.0:0")?
        .start()
        .map_err(|e| anyhow!("Failed to start client error: {e}"))
        .expect("client start error");

    let addr: SocketAddr = format!("{}:4433", LOCALHOST)
        .to_string()
        .parse()
        .expect("SocketAddr parse error");
    // 创建一个链接到 server 的配置，使用 with_server_name 方法指定 DNS 名称
    let connect = Connect::new(addr).with_server_name("localhost");
    let mut conn = client.connect(connect).await?;
    println!("【 conn 】==> {:?}", conn.remote_addr()?);

    // 打开一个双向的 stream
    let stream = conn.open_bidirectional_stream().await?;
    // rx 和 tx 分别是 stream 的读和写
    let (mut rx, mut tx) = stream.split();

    // spawn tokio task copy服务器数据到标准输出
    tokio::spawn(async move {
        let mut stdout = io::stdout();
        if let Err(e) = io::copy(&mut rx, &mut stdout).await {
            println!("Failed to copy data from server. Error: {e}");
        }
        Ok::<(), anyhow::Error>(())
    });

    let mut stdin = io::stdin();
    io::copy(&mut stdin, &mut tx).await?;

    Ok(())
}
