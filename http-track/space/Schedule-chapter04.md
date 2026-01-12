# Chapter 04: 使用 thiserror 进行统一错误处理

## 学习目标

学会使用 `thiserror` 库来创建自定义、统一的错误类型。一个健壮的错误处理系统是构建可靠应用的关键，它能让你清晰地表达错误来源，并方便地在不同模块间传递错误。

## 任务拆解

### Day 1: `thiserror` 核心概念

- [ ] **理论**: 理解 `thiserror` 的核心功能和 `#[derive(Error)]` 宏。
    - `#[error("...")]`: 如何为你的错误类型定义可读的错误信息。
    - `#[from]`: 如何自动地将其他错误类型转换为你的自定义错误类型。这对于组合来自不同库（如 I/O、网络、解析）的错误至关重要。
- [ ] **实践**: 在 `examples/chapter04/error_handling/custom_error.rs` 中进行练习。
    - 定义一个 `AppError` 枚举，包含多种错误变体（例如 `IoError`, `ParseError`）。
    - 为 `AppError` 实现 `std::error::Error` trait（通过派生）。
    - 编写一个函数，它可能会返回不同类型的错误，并观察 `#[from]` 如何简化错误转换。

## 学习资源聚焦

- [ ] `thiserror` 官方文档 (docs.rs)
- [ ] Rust Book - Error Handling 章节

## 需要刻意复习的 Rust 概念

- `enum`
- `Result<T, E>`
- `?` 操作符的工作原理
- `From` 和 `Into` trait
- 派生宏
