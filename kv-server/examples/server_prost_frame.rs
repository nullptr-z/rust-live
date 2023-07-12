use futures::{SinkExt, StreamExt};
use kv_server::{
    CommandRequest, CommandResponse, MemoryDB, PostServerStream, Service, ServiceInner, SledDB,
};
use tokio::net::TcpListener;
use tracing::info;

// fn recevied(cmd: &CommandRequest) {
//     println!("this on_recevied");
// }

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    let addr = "127.0.0.1:7879";
    let listener: TcpListener = TcpListener::bind(addr).await?;
    info!("Start Listener on: http://{}", addr);

    let service: Service<SledDB> = ServiceInner::new(SledDB::default())
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
        let (stream, addr) = listener.accept().await?;
        info!("Client connected http://{}", addr);
        let stream = PostServerStream::new(stream, service.clone());

        tokio::spawn(async move {
            // protobuf 协议，定义如何解析传输的内容
            stream.process().await;

            info!("Client disconnect-: http://{}", addr);
        });
    }
}
