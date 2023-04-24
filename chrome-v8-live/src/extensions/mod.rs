use std::result;

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
        let func = v8::Function::new(scope, print).unwrap();
        // 取一个函数名字
        let name = v8::String::new(scope, "print").unwrap();
        // 将这个函数注册到 v8::Object 上
        bindings.set(scope, name.into(), func.into()).unwrap();
        if let (Ok(result)) = execute_script(scope, GLUE) {
            let func = v8::Local::<v8::Function>::try_from(result).unwrap();
            let value = v8::undefined(scope).into();
            let args = vec![bindings.into()];
            func.call(scope, value, &args).unwrap();
        }
    }
}

fn print(scope: &mut HandleScope, args: FunctionCallbackArguments, mut rv: ReturnValue) {
    let result: serde_json::Value = serde_v8::from_v8(scope, args.get(0)).unwrap();
    println!("rust says {result:#?}");
    rv.set(serde_v8::to_v8(scope, result).unwrap());
}
