mod state;

use state::JsRuntimeState;
use v8::{CreateParams, HandleScope, Isolate, Local, OwnedIsolate, Script, TryCatch, V8};

type LocalValue<'a> = v8::Local<'a, v8::Value>;

pub struct JsRuntime {
    // OwnedIsolate 可以理解为一个Isolate的所有权，他可以借给一个Isolate
    pub isolate: OwnedIsolate,
}

#[derive(Debug, Default)]
pub struct JsRuntimeParams(CreateParams);

impl JsRuntimeParams {
    pub fn new(snapshot: Option<Vec<u8>>) -> Self {
        JsRuntimeParams(Default::default())
    }

    pub fn into_inner(self) -> CreateParams {
        self.0
    }
}

impl JsRuntime {
    pub fn init() {
        // query: thread pool size是做什么的？为什么要设置为0：因为不需要线程池
        // query: idle task runner是做什么的？为什么要设置为false: 因为不需要在后台运行任务
        // make_shared  为什么要用这个？因为需要一个shared_ptr, shared_ptr是一个智能指针，它会自动释放内存
        let platform = v8::new_default_platform(0, false).make_shared();
        V8::initialize_platform(platform);
        V8::initialize();
    }

    pub fn new(params: CreateParams) -> Self {
        let isolate = v8::Isolate::new(params);
        Self::init_isolate(isolate)
    }

    pub fn execute_script(
        &mut self,
        code: impl AsRef<str>,
    ) -> Result<serde_json::Value, serde_json::Value> {
        let context = JsRuntimeState::get_context(&mut self.isolate);
        let handle_scope = &mut HandleScope::with_context(&mut self.isolate, context);

        match execute_script(handle_scope, code) {
            Ok(value) => Ok(serde_v8::from_v8(handle_scope, value).unwrap()),
            Err(err) => Err(serde_v8::from_v8(handle_scope, err).unwrap()),
        }
    }

    pub fn create_snapshot(&mut self) -> Vec<u8> {
        todo!()
    }

    fn init_isolate(mut isolate: OwnedIsolate) -> Self {
        let state = JsRuntimeState::new(&mut isolate);
        isolate.set_slot(state);
        Self { isolate }
    }
}

fn execute_script<'a>(
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
