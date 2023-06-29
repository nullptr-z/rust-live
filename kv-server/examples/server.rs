use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use kv_server::{dispatch, CommandRequest, CommandResponse, MemoryDB};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:7878";
    let listener: TcpListener = TcpListener::bind(addr).await?;
    info!("Start Listener on: http://{}", addr);

    loop {
        let (server_stream, addr) = listener.accept().await?;
        info!("Client connected http://{}", addr);

        let storage = MemoryDB::new();

        tokio::spawn(async move {
            let mut server =
                AsyncProstStream::<_, CommandRequest, CommandResponse, _>::from(server_stream)
                    .for_async();

            while let Some(Ok(cmd)) = server.next().await {
                info!("God a CommandRequest:{:?}", cmd);

                let res = dispatch(cmd, &storage);
                println!("【 res 】==> {:?}", res);

                server.send(res).await.unwrap();
            }
            info!("Client disconnect: http://{}", addr);
        });
    }
}
