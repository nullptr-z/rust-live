use anyhow::Result;
use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use hyper::StatusCode;
use kv_db::{
    memory::MemTable,
    pb::abi::{CommandRequest, CommandResponse},
    Service,
};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9876";
    let listner = TcpListener::bind(addr).await?;
    info!("Listener on {}", addr);

    let server = Service::default();

    loop {
        let (stream, addr) = listner.accept().await?;
        info!("Client {:?} question connected", addr);

        let svc = server.clone();
        tokio::spawn(async move {
            // 使用prost协议来封包
            let mut service =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(stream).for_async();
            while let Some(Ok(msg)) = service.next().await {
                info!("Got a new command {:?}", msg);
                let resp = svc.clone().execute(msg.into());
                info!("execute command response {:?}", resp);
                service.send(resp).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
