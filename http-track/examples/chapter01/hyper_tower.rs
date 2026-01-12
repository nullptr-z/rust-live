//! Tower + Hyper 集成示例
//!
//! 运行: cargo run --example tw-hyper
//! 测试: curl http://127.0.0.1:3000
//!
//! 学习目标:
//! - 将 Tower 中间件应用到 hyper 服务
//! - 理解 hyper 与 tower 的 Service trait 差异
//! - 使用 tower 的内置中间件

use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::TcpListener;
use tower::{Layer, Service, ServiceBuilder};

// ========== 日志中间件（适配 HTTP） ==========
#[derive(Clone)]
pub struct HttpLogLayer;

impl<S> Layer<S> for HttpLogLayer {
    type Service = HttpLogService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        HttpLogService { inner }
    }
}

#[derive(Clone)]
pub struct HttpLogService<S> {
    inner: S,
}

// 为 HTTP 请求实现 Service
//
// 注意类型:
// - Request: hyper::Request<Incoming>
// - Response: hyper::Response<Full<Bytes>>
//
// 关键约束: S 必须能处理 HTTP 请求
impl<S> Service<Request<Incoming>> for HttpLogService<S>
where
    S: Service<Request<Incoming>, Response = Response<Full<Bytes>>, Error = Infallible>
        + Send
        + Clone
        + 'static,
    S::Future: Send,
{
    type Response = Response<Full<Bytes>>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Incoming>) -> Self::Future {
        // TODO: 实现 HTTP 日志
        //
        // 可以记录:
        // - req.method()
        // - req.uri()
        // - req.headers()
        // - 响应状态码
        // - 处理耗时

        let method = req.method().clone();
        let uri = req.uri().clone();
        let start = std::time::Instant::now();

        println!("--> {} {}", method, uri);

        let mut inner = self.inner.clone();
        Box::pin(async move {
            let resp = inner.call(req).await;
            println!(
                "<-- {} {} {:?}",
                method,
                uri,
                start.elapsed()
            );
            resp
        })
    }
}

// ========== 业务处理服务 ==========
#[derive(Clone)]
struct HelloHandler;

impl Service<Request<Incoming>> for HelloHandler {
    type Response = Response<Full<Bytes>>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Incoming>) -> Self::Future {
        Box::pin(async move {
            // TODO: 根据路由返回不同响应
            //
            // 示例:
            // match req.uri().path() {
            //     "/" => Ok(Response::new(Full::new(Bytes::from("Hello, World!")))),
            //     "/health" => Ok(Response::new(Full::new(Bytes::from("OK")))),
            //     _ => Ok(Response::builder()
            //         .status(404)
            //         .body(Full::new(Bytes::from("Not Found")))
            //         .unwrap()),
            // }
            let path = req.uri().path();
            let body = match path {
                "/" => "Hello from Tower + Hyper!",
                "/health" => "OK",
                _ => "Not Found",
            };
            Ok(Response::new(Full::new(Bytes::from(body))))
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("Server running on http://{}", addr);

    // ========== 使用 Tower 构建服务栈 ==========
    //
    // 核心思路:
    // 1. 创建基础 Handler
    // 2. 用 ServiceBuilder 包装中间件
    // 3. 在 accept 循环中为每个连接 clone 服务
    //
    // 注意: hyper 的 serve_connection 需要 Service 实现 Clone
    // 因为每个连接可能处理多个请求

    // TODO: 构建带中间件的服务
    //
    // let svc = ServiceBuilder::new()
    //     .layer(HttpLogLayer)
    //     // 可以添加更多中间件:
    //     // .layer(tower::timeout::TimeoutLayer::new(Duration::from_secs(10)))
    //     .service(HelloHandler);

    loop {
        let (stream, remote_addr) = listener.accept().await?;
        println!("Accepted connection from {}", remote_addr);

        let io = TokioIo::new(stream);

        // TODO: clone 服务并在 spawn 中使用
        // let svc = svc.clone();

        tokio::task::spawn(async move {
            // TODO: 使用 tower service 处理连接
            //
            // 关键: hyper 1.x 的 serve_connection 需要一个闭包或 service_fn
            // 但我们有 Tower Service，需要适配
            //
            // 方式1: 使用 hyper::service::service_fn 包装
            // let hyper_svc = hyper::service::service_fn(|req| {
            //     let mut svc = svc.clone();
            //     async move { svc.call(req).await }
            // });
            //
            // 方式2: 使用 TowerToHyperService 适配器（需要 hyper-util）
            //
            // if let Err(e) = http1::Builder::new()
            //     .serve_connection(io, hyper_svc)
            //     .await
            // {
            //     eprintln!("Error: {}", e);
            // }

            todo!("使用 Tower 中间件处理 HTTP 连接");
        });
    }
}

