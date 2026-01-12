//! 演示如何使用 `thiserror` 创建一个统一的、自定义的错误类型。

use thiserror::Error;

// --- 1. 定义你的自定义错误类型 ---
// 使用 `#[derive(Error, Debug)]` 来让 `thiserror` 为你的枚举实现 `std::error::Error` trait。
#[derive(Error, Debug)]
pub enum AppError {
    // `#[error("...")]` 属性定义了当打印此错误时应显示的文本。
    // 这对于日志记录和用户界面非常有用。
    #[error("I/O error occurred")]
    // `#[from]` 属性是 `thiserror` 的精髓之一。它会自动生成一个 `From<std::io::Error> for AppError` 的实现。
    // 这意味着你可以在返回 `Result<T, AppError>` 的函数中使用 `?` 操作符来处理 `std::io::Result`，
    // `?` 会在 `std::io::Error` 出现时自动将其转换为 `AppError::Io`。
    Io(#[from] std::io::Error),

    #[error("Failed to parse TOML config: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("Hyper error: {0}")]
    Hyper(#[from] hyper::Error),

    #[error("Request URI is invalid: {0}")]
    InvalidUri(#[from] http::uri::InvalidUri),
    
    #[error("An unknown error occurred")]
    Unknown,
}

// 为了方便，我们可以为我们的 Result 定义一个类型别名。
type AppResult<T> = Result<T, AppError>;

// --- 2. 在函数中使用自定义错误类型 ---

// 这个函数模拟从文件加载和解析配置，它可能会产生 I/O 错误或 TOML 解析错误。
fn load_config(path: &str) -> AppResult<String> {
    // `?` 操作符在这里大显身手。
    // `std::fs::read_to_string` 返回 `Result<String, std::io::Error>`。
    // 如果它返回 `Err`，`?` 会自动调用 `AppError::from` 将 `std::io::Error` 转换为 `AppError::Io` 并返回。
    let content = std::fs::read_to_string(path)?;

    // `toml::from_str` 返回 `Result<T, toml::de::Error>`。
    // 同样，如果解析失败，`?` 会自动将 `toml::de::Error` 转换为 `AppError::TomlParse`。
    let _config: toml::Value = toml::from_str(&content)?;
    
    Ok("Config loaded successfully!".to_string())
}

fn main() {
    // --- 3. 匹配和处理错误 ---
    match load_config("default.toml") {
        Ok(message) => println!("Success: {}", message),
        Err(e) => eprintln!("Error: {}", e),
    }

    // 模拟一个不存在的文件，触发 I/O 错误
    match load_config("nonexistent.toml") {
        Ok(message) => println!("Success: {}", message),
        Err(e) => eprintln!("Error: {}", e),
    }

    // 模拟一个格式错误的 TOML 文件，触发解析错误
    std::fs::write("invalid.toml", "port = \"invalid_port").unwrap();
    match load_config("invalid.toml") {
        Ok(message) => println!("Success: {}", message),
        Err(e) => eprintln!("Error: {}", e),
    }
    std::fs::remove_file("invalid.toml").unwrap();
}


// 设计哲学：
// `thiserror` 巧妙地利用了 Rust 的派生宏（`derive`）和 `From` trait。
// 它将定义错误类型的样板代码（例如手动实现 `Display`, `Error`, 和 `From` traits）
// 隐藏在宏的背后，让你能用声明式的方式来定义错误，使代码更清晰、更易于维护。
//
// 你需要将 `thiserror` 添加到 `Cargo.toml` 才能运行此示例：
// thiserror = "1.0"
