use anyhow::Result;
use futures::{SinkExt, StreamExt};
use kv_db::{memory::MemTable, pb::abi::CommandRequest, service_builder::ServiceBuilder};
use prost::Message;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let service = ServiceBuilder::new(MemTable::new()).finish();

    let addr = "127.0.0.1:9876";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connect", addr);
        let svc = service.clone();
        tokio::spawn(async move {
            // 使用 Length Prefix Message封包
            let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
            // 自动解包LPM
            while let Some(Ok(mut buf)) = stream.next().await {
                let cmd = CommandRequest::decode(buf.as_ref()).unwrap();
                info!("got a new cmd request: {:?}", cmd);
                let resp = svc.clone().execute(cmd);
                buf.clear();
                resp.encode(&mut buf).unwrap();
                stream.send(buf.freeze()).await.unwrap();
            }
            info!("Client {:?} disconnect", addr);
        });
    }
}
