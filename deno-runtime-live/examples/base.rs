use std::path::Path;

use deno_core::{anyhow::Context, error::AnyError};
use deno_runtime::{
    deno_core::{self, resolve_path, Extension},
    permissions::{Permissions, PermissionsContainer},
    worker::MainWorker,
};
use deno_runtime_live::MainWorkerOptions;

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let mut options = MainWorkerOptions::default();
    // 禁用一些 runtime 提供的 ops 功能
    let disable_extension = Extension::builder("my_ext")
        .middleware(|op| match op.name {
            // "op_print" => op.disable(),
            // "op_fetch" => op.disable(),
            _ => op,
        })
        .build();
    options.extensions.push(disable_extension);

    // 从文件加载js代码
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/hello_runtime.js");
    let main_module = resolve_path(
        &js_path.to_string_lossy(),
        &std::env::current_dir().context("Unable to get CWD`无法获取当前工作目")?,
    )?;

    // 程序的许多功能和操作都需要获取特定的权限，例如读取文件、网络通信、运行 Web Worker 等等。
    // 为了保障程序的安全性和可靠性，Deno 引入了权限机制，允许程序根据需要获取或放弃各种权限。
    // permissions::PermissionsContainer 就是 Deno 中用于管理和控制权限的核心对象之一
    // --------------------------------------------------
    // PermissionsContainer::allow_all(); 允许所有权限，但一般不推荐
    // 这里我添加了 net 权限，允许程序进行网络通信。
    let permissions = PermissionsContainer::new(Permissions {
        net: Permissions::new_net(&Some(vec![]), false).unwrap(),
        ..Default::default()
    });
    let mut worker =
        MainWorker::bootstrap_from_options(main_module.clone(), permissions, options.into_inner());
    // 传递的参数为 false，则事件循环不会自动退出，需要在代码中手动调用 worker.terminate() 方法来终止循环
    worker.execute_main_module(&main_module).await?;
    worker.run_event_loop(false).await?;

    Ok(())
}
