//! 演示如何从 TOML 文件加载配置到 Rust 结构体中。

use serde::Deserialize;
use std::fs;

// --- 1. 定义与 TOML 文件结构匹配的 Rust 结构体 ---
// 我们再次使用 `Deserialize` 宏。
// 结构体的字段名需要和 TOML 文件中的键名匹配。

#[derive(Deserialize, Debug)]
struct LogConfig {
    level: String,
}

#[derive(Deserialize, Debug)]
struct MiddlewareConfig {
    enable_metrics: bool,
    enable_tracing: bool,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    listen_addr: String,
    port: u16,
    target_url: String,
    log: LogConfig,
    middleware: MiddlewareConfig,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- 2. 读取文件 ---
    
    // 指定配置文件的路径
    let config_path = "default.toml";
    println!("从 '{}' 文件加载配置...", config_path);

    // 使用 `fs::read_to_string` 读取文件的全部内容到一个字符串中。
    // `?` 操作符在这里非常方便，如果文件读取失败，它会自动将错误返回。
    let content = fs::read_to_string(config_path)?;

    // --- 3. 反序列化 ---

    // 使用 `toml::from_str` 将 TOML 格式的字符串解析（反序列化）为 `AppConfig` 结构体。
    // `toml` crate 会自动将 TOML 的表 `[log]` 和 `[middleware]` 映射到
    // `AppConfig` 结构体中的 `log` 和 `middleware` 字段。
    // 如果 TOML 内容的结构与 `AppConfig` 不匹配（例如，缺少字段或类型错误），
    // `from_str` 会返回一个错误。
    let config: AppConfig = toml::from_str(&content)?;

    // --- 4. 使用配置 ---

    println!("\n配置加载成功!");
    println!("{:#?}", config);

    println!("\n应用将在 {}:{} 监听", config.listen_addr, config.port);
    println!("上游目标是: {}", config.target_url);
    println!("日志级别是: {}", config.log.level);

    Ok(())
}

// 你需要将 `serde` (with derive feature) 和 `toml` 添加到 Cargo.toml 才能运行此示例：
// serde = { version = "1.0", features = ["derive"] }
// toml = "0.8"
