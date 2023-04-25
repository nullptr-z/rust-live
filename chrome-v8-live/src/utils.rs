use v8::{HandleScope, Local, Script, ScriptOrigin, TryCatch};

use crate::LocalValue;

/// 执行传入的js代码`code`
pub fn execute_script<'a>(
    scope: &mut HandleScope<'a>,
    code: impl AsRef<str>,
    is_module: bool,
) -> Result<LocalValue<'a>, LocalValue<'a>> {
    let scope = &mut TryCatch::new(scope);
    // js源代码转换成v8源代码
    let source = v8::String::new(scope, code.as_ref()).unwrap();
    // create origin
    // let origin = crete_origin(scope, "dummy.rs", true);
    // 作为一个模块 module 引入并编译，需要使用 compile_module 编译
    // if is_module {
    //     let source = script_compiler::Source::new(source, Some(&origin));
    //     let module = script_compiler::compile_module(scope, source).unwrap();
    //     // 实例化 module
    //     module.instantiate_module(scope, module_callback).unwrap();
    //     let result = module.evaluate(scope).unwrap();
    //     // 得到一个异步 promise 函数
    //     let promise = v8::Local::<v8::Promise>::try_from(result).unwrap();
    //     match promise.state() {
    //         v8::PromiseState::Pending => panic!("pending promise"),
    //         v8::PromiseState::Fulfilled => Ok(promise.result(scope)),
    //         v8::PromiseState::Rejected => Err(promise.result(scope)),
    //     }
    // } else {
    // 作为独立的文件并编译。没有global对象，使用 compile 编译
    // compile编译v8源代码
    Script::compile(scope, source, None)
        // 运行代码
        .and_then(|script| script.run(scope))
        // 返回运行结果
        // 返回Ok是简写了，完整代码是`|value| Ok(value)`
        .map_or_else(|| Err(scope.stack_trace()).unwrap(), Ok)
    // }
}

fn crete_origin<'a>(
    scope: &mut HandleScope<'a>,
    filename: impl AsRef<str>,
    is_module: bool,
) -> ScriptOrigin<'a> {
    todo!()
}

fn module_callback<'a>(
    context: Local<'a, v8::Context>,
    string: Local<'a, String>,
    fixed_array: Local<'a, v8::FixedArray>,
    module: Local<'a, v8::Module>,
) -> Option<v8::Module> {
    println!(
        "context: {context:?}, string: {string:?}, fixed_array: {fixed_array:?}, module: {module:?}"
    );

    // Some(module)
    todo!()
}
