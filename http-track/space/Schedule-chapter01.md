# 本章计划

HTTP 相关库 Hyper 和 Tower 中间件

## ✅ 已完成

### Day 1 - Hyper 基础

- [x] 环境配置（Cargo.toml 依赖）
- [x] hyper Server 基础（examples/chapter01/hyper/service.rs）
  - TcpListener 绑定
  - http1::Builder 处理连接
  - service_fn 包装处理函数
  - TokioIo 适配器
- [x] 理解 hyper 与 tokio 的关系（trait 桥接）
- [x] Client 端使用（examples/chapter01/hyper/client.rs）
<!-- Hyper 深入 -->
- [x] 请求 Headers 读取/设置（examples/chapter01/hyper/headers.rs）
- [x] 请求 Body 处理（POST 数据）
- [x] 简单路由分发（Method + Path 匹配）
- [x] 所有权实践：借用 vs move（`req.collect()` 消费 req）

## 🚧 进行中

### Day 3 - Tower 中间件

- [x] Service trait 核心概念（examples/chapter01/tower/service.rs）
  - poll_ready 背压机制
  - call 返回 Future
  - ServiceExt::ready() / oneshot()
  - tokio::join! vs JoinSet 并发处理异步任务
- [ ] Layer 机制（examples/chapter01/tower/log_layer.rs）
- [ ] 中间件组合（examples/chapter01/tower/compose.rs）
- [ ] Tower + Hyper 集成（examples/chapter01/hyper_tower.rs）

## 📋 待完成

### 代理实现

- [ ] 基础转发代理
- [ ] 请求/响应修改

### 可观测性

- [ ] 指标中间件
- [ ] Prometheus 导出
- [ ] 分布式追踪
