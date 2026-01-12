# Chapter 02: Tracing 结构化日志与追踪

## 学习目标

深入理解 `tracing` 库，学会如何进行结构化日志记录、管理 Span 生命周期，并为后续的分布式追踪和可观测性打下基础。

## 任务拆解

### Day 1: Tracing 基础与 Subscriber

- [ ] **理论**: 理解 `tracing` 的核心概念。
    - `Event` (事件): 最小的日志记录单元。
    - `Span` (范围): 包含多个事件，用于表示操作的持续时间、父子关系。
    - `Subscriber` (订阅者): 接收和处理事件与 Span 的后端。
    - `EnvFilter`: 基于环境变量配置日志级别。
    - `tracing_subscriber` crate: 管理和组合 `Subscriber`。
- [ ] **实践**: 在 `examples/chapter02/tracing/basic_logging.rs` 中实现基本日志功能。
    - 初始化一个简单的 `Subscriber` (例如 `FmtSubscriber`)。
    - 使用 `tracing::info!`, `tracing::warn!`, `tracing::debug!` 等宏发出事件。
    - 了解如何配置日志输出格式。

### Day 2: Span 的使用与上下文管理

- [ ] **理论**: 理解 Span 的生命周期和如何利用它进行上下文追踪。
    - `tracing::span!`: 创建和进入 Span。
    - `Span::enter()`: 进入 Span 的上下文。
    - `Span::exit()`: 退出 Span 的上下文。
    - `async` 函数中的 Span: `#[instrument]` 宏。
- [ ] **实践**: 在 `examples/chapter02/tracing/spans.rs` 中使用 Span。
    - 创建嵌套 Span，观察它们的父子关系。
    - 使用 `#[instrument]` 宏自动为 `async` 函数创建 Span。
    - 观察日志输出中 Span 信息的变化。

### Day 3: 集成与高级话题 (预留)

- [ ] **理论**: 了解 `tracing` 如何与 `tokio`、`hyper` 等异步运行时集成，以及 `OpenTelemetry` 的关系。
- [ ] **实践**: (此阶段暂不编写代码，主要为理论学习和资源查阅)
    - 查阅 `tracing-opentelemetry` 和 `opentelemetry-jaeger` 等库的文档。
    - 思考如何在 Tower 中间件中集成 `tracing` Span。

## 学习资源聚焦

- [ ] `tracing` 官方文档和示例
- [ ] `tracing-subscriber` 文档
- [ ] 相关博客文章和教程

## 需要刻意复习的 Rust 概念

- 宏的使用 (`tracing::info!`, `#[instrument]`)
- 生命周期和所有权 (`tracing` 的内部实现会涉及到一些复杂的引用管理)
- 异步编程 (`Future`, `async/await` 与 Span 的结合)