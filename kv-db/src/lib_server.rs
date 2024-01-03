use anyhow::Result;
use kv_db::sled_db::SledDB;
use kv_db::tls::TlsServerAcceptor;
use kv_db::{service_builder::ServiceBuilder, PostServerStream};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:9876";
    // tudo: 从配置文件中读取
    let server_cert = include_str!("../fixtures/server.cert");
    let server_key = include_str!("../fixtures/server.key");

    let tls = TlsServerAcceptor::new(server_cert, server_key, None)?;
    let service = ServiceBuilder::new(SledDB::new("db/")).finish();
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on{}", addr);

    loop {
        // let tls = tls.clone();
        let (tcp_stream, addr) = listener.accept().await?;
        info!("Clietn {:?} connected", addr);
        // 使用TLS协议包装TCP
        let tls_stream = tls.accept(tcp_stream).await?;
        // 使用Post序列化数据流
        let stream = PostServerStream::new(tls_stream, service.clone());
        tokio::spawn(async move { stream.process().await });
    }
}
