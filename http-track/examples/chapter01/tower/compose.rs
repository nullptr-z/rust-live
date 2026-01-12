//! 中间件组合示例
//!
//! 运行: cargo run --example tw-compose
//!
//! 学习目标:
//! - 使用 ServiceBuilder 组合多个中间件
//! - 理解中间件执行顺序（洋葱模型）
//! - 使用 tower 内置中间件

use std::time::Duration;
use tower::{ServiceBuilder, ServiceExt};

// 复用 log_layer 模块（实际项目中可以 mod 引入）
// 这里为了示例独立，简化实现
mod middleware {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use tower::{Layer, Service};

    // ========== 日志中间件 ==========
    #[derive(Clone)]
    pub struct LogLayer(pub &'static str);

    impl<S> Layer<S> for LogLayer {
        type Service = LogService<S>;
        fn layer(&self, inner: S) -> Self::Service {
            LogService {
                inner,
                tag: self.0,
            }
        }
    }

    pub struct LogService<S> {
        inner: S,
        tag: &'static str,
    }

    impl<S, Req> Service<Req> for LogService<S>
    where
        S: Service<Req> + Send + 'static,
        S::Future: Send,
        Req: std::fmt::Debug + Send + 'static,
    {
        type Response = S::Response;
        type Error = S::Error;
        type Future = Pin<Box<dyn Future<Output = Result<S::Response, S::Error>> + Send>>;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx)
        }

        fn call(&mut self, req: Req) -> Self::Future {
            let tag = self.tag;
            println!("[{}] --> {:?}", tag, req);
            let fut = self.inner.call(req);
            Box::pin(async move {
                let res = fut.await;
                println!("[{}] <-- complete", tag);
                res
            })
        }
    }

    // ========== 计时中间件 ==========
    // TODO: 实现一个简单的计时中间件
    // 记录请求处理耗时
    #[derive(Clone)]
    pub struct TimingLayer;

    impl<S> Layer<S> for TimingLayer {
        type Service = TimingService<S>;
        fn layer(&self, inner: S) -> Self::Service {
            TimingService { inner }
        }
    }

    pub struct TimingService<S> {
        inner: S,
    }

    impl<S, Req> Service<Req> for TimingService<S>
    where
        S: Service<Req> + Send + 'static,
        S::Future: Send,
        Req: Send + 'static,
    {
        type Response = S::Response;
        type Error = S::Error;
        type Future = Pin<Box<dyn Future<Output = Result<S::Response, S::Error>> + Send>>;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx)
        }

        fn call(&mut self, req: Req) -> Self::Future {
            let start = std::time::Instant::now();
            let fut = self.inner.call(req);
            Box::pin(async move {
                let res = fut.await;
                println!("[TIMING] elapsed: {:?}", start.elapsed());
                res
            })
        }
    }
}

use middleware::{LogLayer, TimingLayer};

// 简单的业务服务
async fn handle(req: String) -> Result<String, std::convert::Infallible> {
    // 模拟一些处理时间
    tokio::time::sleep(Duration::from_millis(100)).await;
    Ok(format!("Processed: {}", req))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ========== ServiceBuilder 组合中间件 ==========
    //
    // 核心方法: ServiceBuilder::new()
    //           .layer(outer_layer)  // 最外层
    //           .layer(inner_layer)  // 内层
    //           .service(base_svc)   // 最内层
    //
    // 执行顺序（洋葱模型）:
    //   请求 → outer → inner → base → inner → outer → 响应
    //
    // 类比 HTTP 中间件栈:
    //   [Log] → [Timing] → [Handler] → [Timing] → [Log]

    // TODO: 使用 ServiceBuilder 组合中间件
    //
    // let svc = ServiceBuilder::new()
    //     .layer(LogLayer("OUTER"))     // 第1层: 日志
    //     .layer(TimingLayer)           // 第2层: 计时
    //     .service_fn(handle);          // 核心: 业务逻辑
    //
    // 核心方法: service_fn() - 从 async fn 创建 Service

    // TODO: 调用组合后的服务
    // let resp = svc.oneshot("Hello".to_string()).await?;
    // println!("Response: {}", resp);

    // ========== 观察输出顺序 ==========
    // 预期输出:
    // [OUTER] --> "Hello"
    // [TIMING] elapsed: 100ms
    // [OUTER] <-- complete
    // Response: Processed: Hello

    todo!("组合并调用中间件栈");

    Ok(())
}

