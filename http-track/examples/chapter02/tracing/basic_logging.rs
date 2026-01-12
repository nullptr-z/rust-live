//! 它演示了如何初始化一个简单的 `Subscriber` 并发出不同级别的日志事件。

use tracing::{info, warn, debug, error, instrument};
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    // --- 1. 初始化 Subscriber --- //
    // Subscriber 是 `tracing` 事件的消费者。在这里，我们使用 `FmtSubscriber`，
    // 它会将事件格式化并打印到标准输出。
    // `with_env_filter("info")` 表示默认只显示 info 级别及以上的日志。
    // 你可以通过设置环境变量 `RUST_LOG=debug` 来查看 debug 级别的日志。
    // `init()` 方法设置了全局默认的 Subscriber。
    //
    // 设计哲学：将日志的“生成”和“消费”分离。应用代码只管发出事件，而 Subscriber
    // 负责决定如何处理这些事件（例如打印到控制台、发送到文件、发送到远程服务等）。
    // 这种解耦使得日志系统非常灵活和可配置。
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed");

    info!("应用程序开始运行");
    process_data("some_input_data");
    info!("应用程序完成");
}

#[instrument(level = "info", skip(data_input), fields(input.len = data_input.len()))]
fn process_data(data_input: &str) {
    // `info!` 是一个宏，用于发出 info 级别的日志事件。
    // 它可以像 `println!` 一样接受格式化参数。
    info!("正在处理数据...");

    // `debug!` 宏，默认情况下可能不会显示，除非 RUST_LOG 环境变量设置为 debug 或 trace。
    debug!("数据输入: {}", data_input);

    if data_input.len() > 10 {
        warn!("数据输入过长，可能会影响性能。");
    }

    // 模拟一些处理逻辑
    std::thread::sleep(std::time::Duration::from_millis(50));

    // `error!` 宏用于记录错误事件。
    if data_input == "error" {
        error!("处理数据时发生了一个错误！");
    }

    info!("数据处理完毕。");
}
