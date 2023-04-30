use std::rc::Rc;

use deno_core::{
    anyhow::{Ok, Result},
    resolve_url_or_path,
    serde::de::DeserializeOwned,
    serde_v8, v8, FsModuleLoader, JsRuntime, RuntimeOptions, Snapshot,
};
use lazy_static::lazy_static;
use zstd::decode_all;

lazy_static! {
    /// 代码加载时，加载快照数据
    static ref SNAPSHOT: &'static [u8] = {
        let data = include_bytes!("../snapshots/main.bin");
        let decompressed = decode_all(&data[..]).unwrap().into_boxed_slice();
        Box::leak(decompressed)
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = RuntimeOptions {
        // 指定模块加载器,这里使用的是文件系统加载器，也可以自定义加载器
        module_loader: Some(Rc::new(FsModuleLoader)),
        startup_snapshot: Some(Snapshot::Static(&*SNAPSHOT)),
        ..Default::default()
    };
    let mut rt = JsRuntime::new(options);

    // 加载 JavaScript 代码模块 module
    // 这里需要 FsModuleLoader 模块加载器
    let path = &format!("{}/examples/basic_module.js", env!("CARGO_MANIFEST_DIR"));
    execute_main_module(&mut rt, path).await?;

    // 加载独立 JavaScript 代码
    // let code = include_str!("basic.js");
    // let result: String = eval(&mut rt, code).await?;
    // println!(" {:?}", result);

    Ok(())
}

/// 加载运行独立的JavaScript文件
#[allow(dead_code)]
async fn eval<T>(rt: &mut JsRuntime, code: &str) -> Result<T>
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
async fn execute_main_module(rt: &mut JsRuntime, path: impl AsRef<str>) -> Result<()> {
    let url = resolve_url_or_path(path.as_ref())?;
    // @1 加载js代码模块
    let id = rt.load_main_module(&url, None).await?;
    // @2 执行模块
    let mut received = rt.mod_evaluate(id);
    tokio::select! {
        resolved = &mut received=>{
            resolved.expect("failed to evaluate module`无法评估模块")?;
        }
    // @3 执行事件循环
        _=rt.run_event_loop(false)=>{
            received.await.expect("failed to rvaluate module")?;
        }
    };

    Ok(())
}
