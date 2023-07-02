use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use kv_server::{CommandRequest, CommandResponse, MemoryDB, Service};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:7878";
    let listener: TcpListener = TcpListener::bind(addr).await?;
    info!("Start Listener on: http://{}", addr);

    let service = Service::new(MemoryDB::new());
    loop {
        let (server_stream, addr) = listener.accept().await?;
        info!("Client connected http://{}", addr);

        let svc = service.clone();
        tokio::spawn(async move {
            let mut stream =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(server_stream)
                    .for_async();

            while let Some(Ok(cmd)) = stream.next().await {
                info!("God a CommandRequest: {:?}", cmd);

                let resp = svc.execute(cmd);
                stream.send(resp).await.unwrap();
            }
            info!("Client disconnect: http://{}", addr);
        });
    }
}
