//! 演示 serde 的基本用法，包括 Serialize 和 Deserialize trait。

// 引入 serde 的核心 trait 和派生宏
use serde::{Deserialize, Serialize};

// --- 1. 定义你的数据结构 ---
// 使用 `#[derive(Serialize, Deserialize)]` 可以让 serde 自动为你的结构体实现序列化和反序列化的逻辑。
// `#[derive(Debug)]` 是为了方便我们打印结构体进行调试。
#[derive(Serialize, Deserialize, Debug)]
struct ServerConfig {
    listen_addr: String,
    port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct LogConfig {
    level: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AppConfig {
    // `serde(flatten)` 属性可以将嵌套结构体的字段“拍平”到父结构体中。
    // 在序列化和反序列化时，`ServerConfig` 的字段会表现得就像直接在 `AppConfig` 中一样。
    #[serde(flatten)]
    server: ServerConfig,

    // `log` 字段对应 TOML 中的 `[log]` 表
    log: LogConfig,
}


fn main() {
    // --- 2. 序列化: 将 Rust 结构体转换为数据格式（例如 JSON） ---

    // 创建一个 AppConfig 实例
    let config = AppConfig {
        server: ServerConfig {
            listen_addr: "127.0.0.1".to_string(),
            port: 8080,
        },
        log: LogConfig {
            level: "info".to_string(),
        },
    };

    // 使用 `serde_json::to_string_pretty` 将结构体序列化为格式化的 JSON 字符串。
    // `to_string` 则是序列化为紧凑的单行 JSON。
    // `?` 用于错误处理，如果序列化失败，程序会 panic。
    let json_output = serde_json::to_string_pretty(&config).unwrap();
    println!("---" 序列化为 JSON ---");
    println!("{}", json_output);

    // --- 3. 反序列化: 将数据格式（例如 JSON）转换为 Rust 结构体 ---

    let json_input = r#"
    {
        "listen_addr": "localhost",
        "port": 3000,
        "log": {
            "level": "debug"
        }
    }
    "#;

    // 使用 `serde_json::from_str` 从 JSON 字符串反序列化为 `AppConfig` 实例。
    let deserialized_config: AppConfig = serde_json::from_str(json_input).unwrap();
    println!("\n---" 从 JSON 反序列化 ---");
    println!("{:#?}", deserialized_config);
}

// 设计哲学：
// Serde 的核心是它的 Trait 系统。`Serialize` trait 定义了如何将一个 Rust 类型“分解”成 Serde 的内部数据模型，
// 而 `Deserialize` trait 定义了如何从 Serde 的数据模型“重建”一个 Rust 类型。
// 像 `serde_json`, `toml` 这样的格式库，则负责在它们自己的格式和 Serde 的数据模型之间进行转换。
// 这种设计将“类型如何与 Serde 交互”和“Serde 如何与具体格式交互”这两个问题完全分离开来，
// 使得任何实现了 `Serialize`/`Deserialize` 的类型都可以轻松地在任何支持 Serde 的格式之间转换，极大地提高了代码的重用性。
// 
// 你需要将 `serde` 和 `serde_json` 添加到 `Cargo.toml` 的 `[dependencies]` 中才能运行此示例：
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
