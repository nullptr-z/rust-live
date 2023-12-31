use anyhow::Result;
use kv_db::{service_builder::ServiceBuilder, sled_db::SledDB, PostServerStream, Service};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9876";
    let service = ServiceBuilder::new(SledDB::new("db/")).finish();
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (tcp_stream, addr) = listener.accept().await?;
        info!("Clietn {:?} connected", addr);
        let stream = PostServerStream::new(tcp_stream, service.clone());
        tokio::spawn(async move { stream.process().await });
    }
}
