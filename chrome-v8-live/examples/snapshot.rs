use std::fs;

use chrome_v8_live::JsRuntime;
use clap::{Parser, Subcommand};

const SS_FILE: &str = "./snapshot.blob";

/// 自动实现命令行解析
/// author: 作者
/// version: 版本
/// about: 描述
/// subcommand: 子命令
#[clap(author, version, about)]
#[derive(Parser, Debug)]
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

fn build_snapshot(path: &str) {
    let blob = JsRuntime::create_snapshot();
    fs::write(path, blob).unwrap();
}

fn run_snapshot(_path: &str) {
    todo!()
}
