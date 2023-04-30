/// 加载运行独立的JavaScript文件
use deno_core::{
    anyhow::{Ok, Result},
    serde_v8, v8, JsRuntime,
};

#[tokio::main]
async fn main() -> Result<()> {
    let options = Default::default();
    let mut rt = JsRuntime::new(options);
    let code = include_str!("basic.js");
    // 如果 execute_script 执行的是异步函数，这里会返回一个promise
    let result = rt.execute_script("<anon>", code)?;
    // 如果返回的是 promise，这里需要(resolve)，即rust的await
    // resolve：内部调用 poll event loop, 得到执行后，返回对应状态的值 match PromiseState： resolve， reject， pending， ready， fulfilled
    let result = rt.resolve_value(result).await?;
    // 获取 HandleScope
    let scope = &mut rt.handle_scope();
    // 将 v8 类型转换为 rust 类型的类型，例如JS Promise -> Rust Future
    let result = v8::Local::new(scope, result);
    // 将返回值`反序列化`为对应的类型，这里是字符串字符串
    // let result: String = serde_v8::from_v8(scope, result)?;
    println!(" {:?}", result.is_promise());

    Ok(())
}
