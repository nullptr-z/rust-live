use anyhow::{anyhow, Result};
use s2n_quic::Server;

// 引入证书和私钥
const CERT_PEM: &str = include_str!("../fixtures/cert.pem");
const KEY_PEM: &str = include_str!("../fixtures/key.pem");

#[tokio::main]
async fn main() -> Result<()> {
    // 创建服务器地址，0.0.0.0:4433表示监听本机的 4433 端口
    let addr = "0.0.0.0:4433";
    // 使用 Server::builder() 方法创建服务器，并配置证书和私钥，同时使用 with_io 方法指定服务器地址和端口
    let mut server = Server::builder()
        .with_tls((CERT_PEM, KEY_PEM))?
        .with_io(addr)?
        .start()
        // 使用 map_err 和 anyhow 宏将错误信息包装到 Result 类型中
        .map_err(|e| anyhow!("Failed to start server error: {e}"))?;

    // 打印服务器地址
    println!("Listening on http://{}", addr);

    // 不断从连接池中取出连接，并为每个连接创建一个子线程
    while let Some(mut conn) = server.accept().await {
        println!("accept a connection from {}", conn.remote_addr()?);

        // 为每个连接创建一个线程,从中读取 stream
        tokio::spawn(async move {
            // 不断从连接池中取出一个双向流 stream，直到连接关闭
            while let Some(mut stream) = conn.accept_bidirectional_stream().await? {
                println!(
                    "accept a stream from {}",
                    stream.connection().remote_addr()?
                );
                // 为每个 stream 创建一个子线程，并从中读取数据并发送回去
                tokio::spawn(async move {
                    while let Ok(Some(data)) = stream.receive().await {
                        stream.send(data).await?;
                    }
                    Ok::<(), anyhow::Error>(())
                });
            }
            Ok::<(), anyhow::Error>(())
        });
    }

    Ok(())
}
