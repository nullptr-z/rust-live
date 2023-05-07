pub mod ops;

use deno_core::{
    anyhow::{Ok, Result},
    resolve_url_or_path,
    serde::de::DeserializeOwned,
    serde_v8, v8, JsRuntime,
};

/// 加载运行独立的JavaScript文件
#[allow(dead_code)]
pub async fn eval<T>(rt: &mut JsRuntime, code: &str) -> Result<T>
where
    T: DeserializeOwned,
{
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
    Ok(serde_v8::from_v8(scope, result)?)
}

#[allow(dead_code)]
pub async fn execute_main_module(rt: &mut JsRuntime, path: impl AsRef<str>) -> Result<()> {
    let url = resolve_url_or_path(path.as_ref())?;
    // @1 加载js代码模块
    let id = rt.load_main_module(&url, None).await?;
    // @2 执行模块
    let mut received = rt.mod_evaluate(id);
    // @3 执行事件循环
    loop {
        tokio::select! {
            resolved = &mut received=>{
               return resolved.expect("failed to evaluate module`无法评估模块");
            }
            _=rt.run_event_loop(false)=>{
                // received.await.expect("failed to rvaluate module")?;
            }
        };
    }
}
