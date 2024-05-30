mod redis_core;

use anyhow::Result;
use redis_core::command;
use std::io;
use tokio::{
    io::AsyncWriteExt as _,
    net::{TcpListener, TcpStream},
};
use tracing::{error, info};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:6379";
    let lis = TcpListener::bind(addr).await?;
    info!("listening on {:?}", addr);

    loop {
        let (stream, addr) = lis.accept().await?;
        tokio::spawn(async move {
            info!("accepted connection from {:?}", addr);
            if let Err(e) = process_redis_task(stream).await {
                error!("[process_redis_task] error {:?}", e);
            };
            info!("connected over {:?}", addr);
        });
    }
}

async fn process_redis_task(mut stream: TcpStream) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);
        match stream.try_read_buf(&mut buf) {
            Ok(0) => {
                return Ok(());
            }
            Ok(n) => {
                let cmd_str = String::from_utf8_lossy(&buf[..n]);
                info!("{:?}", cmd_str);
                let cmd_line = command::CmdLine::new_from_str(&cmd_str)?;
                info!("{:?}", cmd_line);
                let resp = b"+PONG\r\n";
                // stream.writable().await?;
                stream.write_all(resp).await?;
                info!("{:?}", String::from_utf8_lossy(resp));
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
}
