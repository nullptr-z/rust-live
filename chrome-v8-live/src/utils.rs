use v8::{HandleScope, Script, TryCatch};

use crate::LocalValue;

/// 执行传入的js代码`code`
pub fn execute_script<'a>(
    scope: &mut HandleScope<'a>,
    code: impl AsRef<str>,
) -> Result<LocalValue<'a>, LocalValue<'a>> {
    let scope = &mut TryCatch::new(scope);
    // js源代码转换成v8源代码
    let source = v8::String::new(scope, code.as_ref()).unwrap();
    // compile编译v8源代码
    Script::compile(scope, source, None)
        // 运行代码
        .and_then(|script| script.run(scope))
        // 返回运行结果
        // 返回Ok是简写了，完整代码是`|value| Ok(value)`
        .map_or_else(|| Err(scope.stack_trace()).unwrap(), Ok)
}
