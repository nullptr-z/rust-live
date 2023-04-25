use v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

use crate::utils::execute_script;

pub struct Extensions {
    _name: String,
    _source: String,
}

const GLUE: &str = include_str!("glue.js");

impl Extensions {
    pub fn install(scope: &mut HandleScope) {
        // 创建 v8::Object, 用于存储扩展
        let bindings = v8::Object::new(scope);
        // 创建一个函数
        let print_func = v8::Function::new(scope, print).unwrap();
        // 取一个函数名字
        let name = v8::String::new(scope, "print").unwrap();
        // 将这个函数注册到 v8::Object 上
        bindings.set(scope, name.into(), print_func.into()).unwrap();

        let fetch_func = v8::Function::new(scope, fetch).unwrap();
        let name = v8::String::new(scope, "fetch").unwrap();
        bindings.set(scope, name.into(), fetch_func.into()).unwrap();

        // 将 bindings 对象传入到 glue.js 中
        if let Ok(result) = execute_script(scope, GLUE, false) {
            // 从 result 提取函数对象，然后调用
            let func = v8::Local::<v8::Function>::try_from(result).unwrap();
            let recv = v8::undefined(scope).into();
            // bindings 中存储了所有的扩展函数和函数名，参数会传入对应的函数中
            let args = vec![bindings.into()];
            // recv 表示函数的接收者（即 this 对象）s
            func.call(scope, recv, &args).unwrap();
        }
    }
}

fn print(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    let result: serde_json::Value = serde_v8::from_v8(scope, args.get(0)).unwrap();
    println!("rust says {result:#?}");
    //
    rv.set(serde_v8::to_v8(scope, result).unwrap());
}

/// FunctionCallbackArguments:
/// 它有多个重载版本，可以根据参数类型和数量来获取参数值。
///  还提供了其他方法，例如 Length() 方法用于获取参数数量
/// This() 方法用于获取函数调用的 this 对象等。
/// 这些方法都可以帮助开发者在函数回调中获取相关的参数和信息。
fn fetch(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    let url: String = serde_v8::from_v8(scope, args.get(0)).unwrap();
    let result = reqwest::blocking::get(&url).unwrap().text().unwrap();
    rv.set(serde_v8::to_v8(scope, result).unwrap());
}
