use chrome_v8_live::JsRuntime;
use clap::{Parser, Subcommand};
use std::fs;

const SS_FILE: &str = "./snapshot.blob";

#[derive(Parser, Debug)]
/// 自动实现命令行解析
/// author: 作者
/// version: 版本
/// about: 描述
/// subcommand: 子命令
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    Build,
    Run,
}

fn main() {
    // 初始化v8引擎 JsRuntime
    JsRuntime::init();

    let args = Args::parse();
    match args.action {
        Action::Build => build_snapshot(SS_FILE),
        Action::Run => run_snapshot(SS_FILE),
    }
}

// 吧JsRuntime的快照保存到文件中，保存为二进制文件, 这样下次就可以直接加载快照了
fn build_snapshot(path: &str) {
    let blob = JsRuntime::create_snapshot();
    fs::write(path, blob).unwrap();
}

fn run_snapshot(path: &str) {
    let blob = fs::read(path).unwrap();
    let mut runtime = JsRuntime::new(Some(blob));
    let code = r#"
      function hello(){
        print({a:"Hello ",b:"World",c:"我的天那"});
        return fetch("https://www.rust-lang.org/");
      }
      hello();
    "#;
    let result = runtime.execute_script(code, true);
    println!("\n\nexecute_script result: {:?}", result);
}
