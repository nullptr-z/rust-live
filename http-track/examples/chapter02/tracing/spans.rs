//! Span 用于表示一段操作的范围，可以包含多个事件，并形成父子关系，
//! 对于理解复杂系统的执行流程和分布式追踪非常有用。

use tracing::{
    info,
    span,
    warn,
    Level,
    instrument
};
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    // --- 1. 初始化 Subscriber --- //
    // 同样，我们需要初始化一个 Subscriber 来接收和处理 Span 和 Event。
    // 这里我们将默认级别设置为 debug，以便能看到更多的细节。
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env().add_directive("debug".parse().unwrap()))
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed");

    info!("应用程序启动");

    // 调用一个会创建并进入 Span 的函数
    perform_complex_operation(5);

    info!("应用程序退出");
}

// `#[instrument]` 宏会自动为函数创建一个 Span。
// 当函数被调用时，Span 会自动进入；函数返回时，Span 会自动退出。
// `level` 参数指定 Span 的级别。
// `name` 参数可以给 Span 一个自定义名称。
// `fields` 参数允许我们向 Span 添加额外的字段，这些字段会伴随 Span 的整个生命周期。
#[instrument(level = "info", name = "complex_op", fields(param_value = input_value))]
fn perform_complex_operation(input_value: u32) {
    info!("开始执行复杂操作");

    // 手动创建一个 Span
    let inner_span = span!(Level::WARN, "inner_task", task_id = 1);

    // `enter()` 方法进入 Span 的上下文。当 `_guard` 离开作用域时，Span 会自动退出。
    let _guard = inner_span.enter();

    info!("在内部任务中执行一些工作...");
    std::thread::sleep(std::time::Duration::from_millis(100));
    warn!("内部任务中发生了警告");

    // _guard 离开作用域，inner_span 退出。

    // 再次调用带有 #[instrument] 的函数，观察嵌套 Span 的行为
    another_sub_operation();

    info!("复杂操作完成");
}

#[instrument(level = "debug")]
fn another_sub_operation() {
    debug!("执行另一个子操作");
    std::thread::sleep(std::time::Duration::from_millis(70));
    debug!("子操作完成");
}
