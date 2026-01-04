## TUDO

-[] 项目初始化 - 创建 Cargo.toml 和基础目录结构
-[] 配置系统 - 实现 config.rs 和 default.toml 配置加载
-[] 错误处理 - 设计统一的 error.rs 错误类型
-[] HTTP 客户端 - 实现 proxy/client.rs HTTP 客户端封装
-[] 核心代理服务 - 实现 proxy/service.rs 基础代理功能
-[] 中间件框架 - 实现 middleware/layer.rs 中间件 Layer 机制
-[] 日志中间件 - 实现 middleware/log.rs 结构化日志
-[] 指标中间件 - 实现 middleware/metrics.rs 指标收集
-[] 追踪中间件 - 实现 middleware/trace.rs 分布式追踪
-[] 限流中间件 - 实现 middleware/rate_limit.rs 限流功能
-[] Prometheus 导出 - 实现 telemetry/metrics_exporter.rs
-[] OpenTelemetry 追踪 - 实现 telemetry/tracer.rs 配置
-[] 编写示例 - 创建 basic_proxy.rs 和 with_middleware.rs
-[] 单元测试 - 编写 middleware_tests.rs 中间件测试
-[] 集成测试 - 编写 integration_tests.rs 端到端测试

observable-proxy/
├── Cargo.toml
├── config/
│ ├── default.toml # 默认配置
│ └── config.rs # 配置结构体定义
├── src/
│ ├── main.rs # 应用入口点
│ ├── lib.rs # 库主文件
│ ├── error.rs # 统一错误处理
│ ├── proxy/
│ │ ├── mod.rs
│ │ ├── client.rs # HTTP 客户端封装
│ │ └── service.rs # 核心代理服务实现
│ ├── middleware/ # ★ 中间件核心目录
│ │ ├── mod.rs
│ │ ├── layer.rs # 中间件 Layer 定义
│ │ ├── log.rs # 结构化日志中间件
│ │ ├── metrics.rs # 指标收集中间件
│ │ ├── trace.rs # 分布式追踪中间件
│ │ ├── rate_limit.rs # 限流中间件
│ │ ├── auth.rs # 认证中间件（可选）
│ │ └── cache.rs # 缓存中间件（可选）
│ └── telemetry/
│ ├── mod.rs
│ ├── metrics_exporter.rs # Prometheus 指标导出
│ └── tracer.rs # OpenTelemetry 追踪设置
├── examples/ # 使用示例
│ ├── basic_proxy.rs
│ └── with_middleware.rs
└── tests/
├── integration_tests.rs # 集成测试
└── middleware_tests.rs # 中间件单元测试

## Dependencies

```toml
[dependencies]
hyper = { version = "1.0", features = ["client", "server", "http1", "http2"] }
tower = { version = "0.4", features = ["full"] }
tokio = { version = "1.0", features = ["full"] }
tracing = { version = "0.1", features = ["log"] }
tracing-subscriber = "0.3"
metrics = "0.21"
metrics-exporter-prometheus = "0.13"
opentelemetry = { version = "0.22", features = ["rt-tokio"] }
opentelemetry-jaeger = "0.22"
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"
```


## 实现路线图（四阶段）
阶段1：基础代理 (复习核心语法)
实现能转发HTTP请求的简单代理

熟悉 hyper 客户端/服务器基本用法

阶段2：单一中间件 (复习泛型/trait)
选择实现一个中间件（如日志）

理解 Service trait 和中间件模式

阶段3：中间件管道 (复习组合模式)
实现多个中间件（日志、指标、限流）

使用 ServiceBuilder 组合中间件

测试中间件执行顺序

阶段4：生产化 (复习异步/生态)
添加配置管理

实现可观测性端点（如 /metrics）

编写集成测试

## 需要刻意复习的 Rust 概念

所有权/借用: 在中间件间传递请求/响应

泛型/trait 约束: impl<S> Service<R> for LogMiddleware<S> where S: Service<R>

异步编程: async/await、Future、Pin<Box<dyn Future>>

智能指针: Arc、Mutex 用于共享状态（如限流器）

错误处理: Result、? 操作符、错误类型转换

模块系统: pub 可见性、模块组织

## 测试策略

单元测试: 每个中间件独立测试

集成测试: 启动代理服务器，使用 reqwest 发送测试请求

属性测试: 测试限流器的正确性

## 学习资源聚焦

Tower 文档: 理解 Service、Layer、Middleware 概念

hyper 示例: 参考官方代理示例

tracing 文档: 学习结构化日志和 Span

## 关键设计点

1. 代理核心 (proxy/service.rs)
   核心结构: ProxyService，包装 hyper::Client

关键操作: 修改请求 URI 指向上游，转发请求

复习要点: 泛型约束、Service trait 实现、async/await、错误类型擦除

2. 中间件通用模式 (middleware/)
   每个中间件包含：

中间件结构体 (如 LogMiddleware<S>): 包装内部服务

对应的 Layer (如 LogLayer): 用于 ServiceBuilder 组合

自定义 Future (可选): 用于在请求前后执行逻辑

3. 中间件实现要点
   日志中间件 (middleware/log.rs)
   功能: 记录请求开始/结束、方法、路径、耗时、状态

复习点: tracing 库使用、Span 生命周期、结构化日志

关键结构: LogMiddleware 和 LogFuture

指标中间件 (middleware/metrics.rs)
功能: 统计请求数、成功率、延迟分布

复习点: metrics 库、原子操作、直方图

关键指标: requests_total、request_duration_seconds

限流中间件 (middleware/rate_limit.rs)
算法: 令牌桶算法

复习点: Arc<Mutex<T>> 共享状态、原子操作、自定义错误响应

关键结构: TokenBucket、RateLimitMiddleware

追踪中间件 (middleware/trace.rs)
功能: 生成/传播 Trace ID，链路追踪

复习点: 请求头操作、上下文传播

4. 中间件组合 (middleware/mod.rs)
   构建器模式: MiddlewareBuilder 或直接使用 tower::ServiceBuilder

执行顺序: 中间件按添加顺序形成“洋葱模型”

复习点: Tower 生态的 Layer/Service 模式

5. 配置管理 (config.rs)
   配置结构: ProxyConfig 包含服务器、目标、中间件配置

复习点: serde 反序列化、Default trait 实现、配置文件热重载

6. 错误处理 (error.rs)
   自定义错误: ProxyError enum，覆盖网络、解析、限流等错误

复习点: thiserror 宏、错误转换、Result 类型别名


