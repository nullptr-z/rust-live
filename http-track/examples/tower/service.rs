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
impl Service<String> for HelloService {
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
        todo!("返回 Poll::Ready(Ok(()))")
    }

    fn call(&mut self, req: String) -> Self::Future {
        // TODO: 处理请求
        // 1. 可以在这里做同步处理
        // 2. 返回的 Future 中做异步处理
        //
        // 提示: Box::pin(async move { Ok(format!("Hello, {}!", req)) })
        todo!("处理请求并返回 Future")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut svc = HelloService;

    // ========== 调用服务的几种方式 ==========

    // 方式1: 手动 ready + call
    // 核心方法: ServiceExt::ready() - 等待 poll_ready 返回 Ready
    // let svc = svc.ready().await?;
    // let resp = svc.call("World".to_string()).await?;

    // 方式2: oneshot - 一次性调用（消费 service）
    // 核心方法: ServiceExt::oneshot() - ready + call 组合
    // let resp = svc.oneshot("World".to_string()).await?;

    // TODO: 选择一种方式调用服务，打印响应
    todo!("调用 HelloService");

    Ok(())
}

