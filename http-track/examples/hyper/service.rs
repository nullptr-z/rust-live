use http_body_util::Full;
use hyper::body::Bytes;
use hyper::{Request, Response, body::Incoming, server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is running on http://0.0.0.0:3000");

    loop {
        let (stream, _) = server.accept().await?;
        // hyper	纯 HTTP 协议实现，定义 hyper::rt::Read/Write trait
        // hyper-util	提供 TokioIo 适配器，桥接 tokio 的 AsyncRead/AsyncWrite
        // 这样 hyper 可以支持任何异步运行时，不只是 tokio。
        let io = TokioIo::new(stream); // ← 包装 stream
        let service = service_fn(hello_world);
        tokio::task::spawn(async move {
            if let Err(e) = http1::Builder::new().serve_connection(io, service).await {
                println!("Error serving connection: {}", e);
            }
        });
    }
}

// 处理函数
async fn hello_world(_req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
