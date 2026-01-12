//! 日志中间件示例
//!
//! 运行: cargo run --example tw-log-layer
//!
//! 学习目标:
//! - 理解 Layer trait 的工厂模式
//! - 实现 Service 委托/包装模式
//! - 理解中间件的洋葱模型

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread::sleep;
use tower::{Layer, Service};

// ========== 第一步: 定义 Layer ==========
//
// Layer 是创建中间件的工厂
// 它持有中间件的配置，调用 layer() 时包装内部服务
#[derive(Clone)]
pub struct LogLayer {
    target: &'static str, // 日志标签，如 "HTTP", "RPC"
}

impl LogLayer {
    pub fn new(target: &'static str) -> Self {
        Self { target }
    }
}

// 为 LogLayer 实现 Layer trait
//
// 核心方法: layer(&self, inner: S) -> Self::Service
// - inner: 被包装的内部服务
// - 返回: 包装后的新服务
//
// 设计思考:
// - 为什么 Layer 和 Service 分开？ → 关注点分离，配置与实例化解耦
// - S 是泛型，意味着可以包装任意服务
impl<S> Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        // TODO: 创建 LogService，传入 inner 和配置
        LogService {
            inner,
            target: self.target,
        }
    }
}

// ========== 第二步: 定义包装后的 Service ==========
pub struct LogService<S> {
    inner: S,
    target: &'static str,
}

// 为 LogService 实现 Service trait
//
// 关键约束: S: Service<Request>
// 这是 Rust trait bound 的精髓：
// "只要 S 能处理 Request，我包装后也能处理 Request"
//
// 中间件模式:
// 1. 前置处理（打印请求日志）
// 2. 调用内部服务
// 3. 后置处理（打印响应日志）
impl<S, Request> Service<Request> for LogService<S>
where
    S: Service<Request> + Send,
    S::Future: Send + 'static,
    Request: std::fmt::Debug, // 约束: 请求必须可打印
{
    type Response = S::Response;
    type Error = S::Error;
    // TODO: Future 类型
    // 如果只需要前置日志: 直接用 S::Future
    // 如果需要后置日志: 需要包装 Future
    type Future = Pin<Box<dyn Future<Output = Result<S::Response, S::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // TODO: 委托给内部服务
        // 中间件通常直接转发 poll_ready
        // 核心方法: self.inner.poll_ready(cx)
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let target = self.target.to_owned();
        // 1. 可以对 req 加工处理
        let future = self.inner.call(req);
        // 2. 包装 future，在完成时打印响应
        Box::pin(async move {
            println!("[{}] <-- before Response", target);
            let resp = future.await;
            println!("[{}] <-- after Response: {:?}", target, resp.is_ok());
            resp
        })
        // 思考: 为什么需要 move？target 和 future 的所有权去哪了？
    }
}

// ========== 测试用的内部服务 ==========
struct TestService;

impl Service<String> for TestService {
    type Response = String;
    type Error = std::convert::Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<String, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: String) -> Self::Future {
        Box::pin(async move {
            // 指定秒数后返回 Hello
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            Ok(format!("TestService recevie a req: {}", req))
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ========== 使用 Layer 包装服务 ==========
    //
    // 方式1: 这种方式需要自己使用 Layer Trait，但是更灵活
    let layer = LogLayer::new("my tower loger middleware");
    let mut service: LogService<TestService> = layer.layer(TestService);
    let call = service.call("send a request to TestService call func".to_owned());
    let reps = tokio::spawn(call).await;
    if reps.is_ok() {
        println!("{:?}", reps?)
    }

    // 方式2: 使用 ServiceBuilder，大多时候直接使用这种方式也是可以的
    // use tower::ServiceBuilder;
    // let service = ServiceBuilder::new()
    //     .layer(LogLayer::new("TEST"))
    //     .service(EchoService);

    Ok(())
}
