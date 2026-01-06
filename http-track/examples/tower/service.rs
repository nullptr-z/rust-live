//! Tower Service 基础示例
//!
//! 运行: cargo run --example tw-service
//!
//! 学习目标:
//! - 理解 Service trait 的核心方法
//! - 实现自定义 Service
//! - 使用 ServiceExt 便捷方法

use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower::{Service, ServiceExt};

// ========== 定义请求结构体 ==========
struct HelloRequest {
    name: String,
    delay_secs: u64,
}

// ========== 定义服务结构体 ==========
struct HelloService;

// ========== 实现 Service trait ==========
//
// 核心方法解释:
// - poll_ready(): 背压机制，返回 Poll::Ready(Ok(())) 表示可以接收请求
//                 如果服务繁忙（如连接池满），返回 Poll::Pending
// - call(): 处理请求，返回 Future
//
// 设计思考:
// - 为什么 call 接收 &mut self？ → 允许服务维护内部状态
// - 为什么返回 Future 而不是 async fn？ → trait 中不能直接用 async fn（截至目前）
impl Service<HelloRequest> for HelloService {
    type Response = String;
    type Error = Infallible; // std::convert::Infallible 表示永不出错
    // TODO: 选择 Future 类型
    // 方式1: std::future::Ready<Result<String, Infallible>> - 同步立即返回
    // 方式2: Pin<Box<dyn Future<Output = Result<String, Infallible>> + Send>> - 支持 async
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // TODO: 实现就绪检查
        // 简单服务直接返回 Ready
        // 复杂服务可能需要检查资源（连接池、限流器等）
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: HelloRequest) -> Self::Future {
        // TODO: 处理请求，Future 决定同步返回还是，还是 async
        // 1. 可以在这里做同步处理
        // 2. 返回的 Future 中做异步处理

        // 这里选择返回一个 async
        Box::pin(async move {
            // 指定秒数后返回 Hello
            tokio::time::sleep(std::time::Duration::from_secs(req.delay_secs)).await;
            Ok(format!("Hello, {} (after {}s)", req.name, req.delay_secs))
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 核心方法: ServiceExt::ready() - 等待 poll_ready 返回 Ready
    let mut svc = HelloService;
    let svc = svc.ready().await?;

    // ========== 调用服务的几种方式 ==========
    // 方式1: 手动 ready + call
    let req = HelloRequest {
        name: "World1".into(),
        delay_secs: 1,
    };
    let resp = svc.call(req).await?;
    println!("resp: {}", resp);

    // 方式2: oneshot - 一次性调用（消费 service）
    // 核心方法: ServiceExt::oneshot() - ready + call 组合
    // 注意: oneshot 会消费 service，之后不能再使用
    let req = HelloRequest {
        name: "World2".into(),
        delay_secs: 2,
    };
    let resp = HelloService.oneshot(req).await?;
    println!("resp: {}", resp);

    // 方式3: 并发调用多个请求
    // 返回时间取决于最慢的那个请求的完成时间，被阻塞的最慢的请求会阻塞整个 join 的完成
    let (r1, r2, r3) = tokio::join!(
        HelloService.oneshot(HelloRequest {
            name: "World3".into(),
            delay_secs: 3,
        }),
        HelloService.oneshot(HelloRequest {
            name: "World4".into(),
            delay_secs: 2,
        }),
        HelloService.oneshot(HelloRequest {
            name: "World5".into(),
            delay_secs: 1,
        }),
    );
    println!("并发结果: {:?}, {:?}, {:?}", r1?, r2?, r3?);

    // 方式4: JoinSet - 按完成顺序处理
    // 哪个先返回就先打印哪个，不会被最慢的阻塞
    // 每个 future 会 spawn 为独立的 tokio task
    println!("\n--- JoinSet: 按完成顺序输出 ---");
    let mut set = tokio::task::JoinSet::new();
    set.spawn(HelloService.oneshot(HelloRequest {
        name: "World8-3s".into(),
        delay_secs: 3,
    }));
    set.spawn(HelloService.oneshot(HelloRequest {
        name: "World7-2s".into(),
        delay_secs: 2,
    }));
    set.spawn(HelloService.oneshot(HelloRequest {
        name: "World6-1s".into(),
        delay_secs: 1,
    }));

    // 按完成顺序逐个处理
    while let Some(result) = set.join_next().await {
        println!("完成: {:?}", result??); // 双 ? : JoinError + ServiceError
    }

    Ok(())
}
