# Chapter 03: 使用 Serde 和 TOML 进行配置管理

## 学习目标

学会使用 `serde` 库对 Rust 数据结构进行序列化和反序列化，并结合 `toml` 库，实现从配置文件加载应用配置的功能。这是构建可配置、可维护应用的基石。

## 任务拆解

### Day 1: `serde` 核心概念

- [ ] **理论**: 理解 `serde` 的核心 trait：`Serialize` 和 `Deserialize`。
    - `#[derive(Serialize, Deserialize)]`: `serde` 如何通过派生宏自动实现这些 trait。
    - `serde` 的数据模型：`serde` 如何在不同的数据格式（JSON, TOML, YAML 等）和 Rust 结构体之间建立一个抽象的桥梁。
    - 常用属性：`#[serde(rename = "...")]`, `#[serde(default)]` 等。
- [ ] **实践**: 在 `examples/chapter03/config/basic_serde.rs` 中进行练习。
    - 定义一个配置结构体 `AppConfig`。
    - 使用 `serde_json`（作为示例）将其序列化为 JSON 字符串，再反序列化回来。

### Day 2: 解析 TOML 文件

- [ ] **理论**: 了解 `toml` crate 的基本用法。
    - `toml::from_str`: 如何将 TOML 格式的字符串解析成 `serde` 兼容的结构体。
    - 了解 TOML 格式的语法：键值对、表（tables）、数组表（arrays of tables）。
- [ ] **实践**: 在 `examples/chapter03/config/load_toml.rs` 中实现加载功能。
    - 创建一个 `default.toml` 配置文件。
    - 编写代码读取 `default.toml` 文件内容。
    - 使用 `toml::from_str` 将文件内容解析到你定义的 `AppConfig` 结构体中。

## 学习资源聚焦

- [ ] `serde` 官方文档和网站 (serde.rs)
- [ ] `toml` crate (docs.rs)
- [ ] `serde` 官方示例

## 需要刻意复习的 Rust 概念

- 派生宏（`#[derive(...)]`）
- `Result` 和 `?` 操作符（文件 I/O 和解析都可能失败）
- 模块系统（如何组织配置相关的代码）
- 字符串处理 (`String`, `&str`)
