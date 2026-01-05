# Tower 中间件学习计划

## 食用指南

运行命令：

- Service 基础 cargo run --example tw-service
- 日志中间件 cargo run --example tw-log-layer
- 中间件组合 cargo run --example tw-compose
- Tower + Hyper cargo run --example tw-hyper

建议学习顺序：

1.tw-service - 先填充 todo!()，理解 poll_ready 和 call
2.tw-log-layer - 实现 Layer 包装模式
3.tw-compose - 体验 ServiceBuilder 的便捷性
4.tw-hyper - 把中间件应用到真实 HTTP 服务

## 目标

通过 4 个递进的示例，掌握 Tower 的 Service/Layer 抽象和中间件模式。

## 学习路径

```mermaid
flowchart LR
    A[1. Service trait] --> B[2. 实现 LogLayer]
    B --> C[3. 中间件组合]
    C --> D[4. 集成 hyper]
```

## 核心概念

### Service Trait - 请求/响应抽象

```rust
pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn call(&mut self, req: Request) -> Self::Future;
}
```

**设计巧妙之处：**

- `Request` 作为泛型参数 → 一个 Service 可以处理任意请求类型
- `poll_ready` → 背压机制，服务可以告诉调用者"我忙，稍等"
- `Future` 关联类型 → 避免 `Box<dyn Future>` 堆分配，零成本抽象

### Layer Trait - 中间件工厂

```rust
pub trait Layer<S> {
    type Service;
    fn layer(&self, inner: S) -> Self::Service;
}
```

**为什么需要 Layer？**

- 解耦中间件配置与服务实例化
- 支持 `ServiceBuilder` 链式组合

## Step 1: 实现自定义 Service

在 `examples/tower/service.rs` 中：

- 定义 `HelloService` 结构体
- 实现 `Service<String>` trait
- 学习 `poll_ready`、`call` 方法语义
- 使用 `ServiceExt::ready()` 调用服务

## Step 2: 实现日志中间件

在 `examples/tower/log_layer.rs` 中：

- 定义 `LogLayer` 结构体
- 实现 `Layer<S>` trait
- 定义 `LogService<S>` 包装类型
- 学习 Service 委托模式

## Step 3: 中间件组合

在 `examples/tower/compose.rs` 中：

- 使用 `ServiceBuilder::new().layer().layer().service()`
- 理解中间件执行顺序（洋葱模型）
- 尝试 `tower::timeout::TimeoutLayer`

## Step 4: 集成 hyper

在 `examples/tower/hyper_tower.rs` 中：

- 用 Tower 中间件包装 hyper 服务
- 使用 `tower::ServiceBuilder` 添加日志、超时
- 对比 `service_fn` 和 `TowerToHyperService`

## 需要更新的文件

1. [Cargo.toml](../Cargo.toml) - 添加 tower 依赖
2. 创建 `examples/tower/` 目录及示例文件

## 依赖配置

```toml
[dependencies]
tower = { version = "0.5", features = ["full"] }
# 已有
hyper = { version = "1.6", features = ["full"] }
hyper-util = { version = "0.1", features = ["full"] }
http-body-util = "0.1"
tokio = { version = "1", features = ["full"] }
```

## 关键 API 速查

| 方法/类型               | 来源  | 用途                  |
| ----------------------- | ----- | --------------------- |
| `Service::poll_ready()` | tower | 检查服务是否就绪      |
| `Service::call()`       | tower | 处理请求              |
| `ServiceExt::ready()`   | tower | 等待服务就绪（async） |
| `ServiceExt::oneshot()` | tower | ready + call 一步完成 |
| `Layer::layer()`        | tower | 包装内部服务          |
| `ServiceBuilder::new()` | tower | 构建中间件栈          |
| `service_fn()`          | tower | 从闭包创建 Service    |

## 思考题

1. `poll_ready` 返回 `Pending` 时会发生什么？调用者应该怎么处理？
2. 为什么 `call(&mut self, req)` 需要 `&mut self`？这对并发有什么影响？
3. 如何让一个 Service 可以被多个任务共享？（提示：`Buffer`、`Clone`）
