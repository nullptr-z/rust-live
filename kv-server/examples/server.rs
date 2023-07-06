use async_prost::AsyncProstStream;
use futures::{SinkExt, StreamExt};
use kv_server::{CommandRequest, CommandResponse, MemoryDB, Service, ServiceInner};
use tokio::net::TcpListener;
use tracing::info;

// fn recevied(cmd: &CommandRequest) {
//     println!("this on_recevied");
// }

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:7878";
    let listener: TcpListener = TcpListener::bind(addr).await?;
    info!("Start Listener on: http://{}", addr);

    let service: Service = ServiceInner::new(MemoryDB::new())
        .fn_recevied(|_req| {
            println!("this on_recevied");
        })
        .fn_executed(|_res| {
            println!("this executed");
        })
        .fn_before_send(|_res| {
            println!("this before_send");
        })
        .fn_after_send(|| {
            println!("this after_send");
        })
        .into();
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
