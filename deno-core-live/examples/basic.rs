use std::rc::Rc;

use deno_core::{
    anyhow::{Ok, Result},
    FsModuleLoader, JsRuntime, RuntimeOptions, Snapshot,
};
use deno_core_live::execute_main_module;
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
