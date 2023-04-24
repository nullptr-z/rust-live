mod extensions;
mod state;
mod utils;

use extensions::Extensions;
use state::JsRuntimeState;
use utils::execute_script;
use v8::{CreateParams, HandleScope, OwnedIsolate, V8};

type LocalValue<'a> = v8::Local<'a, v8::Value>;

pub struct JsRuntime {
    // OwnedIsolate 可以理解为一个Isolate的所有权，他可以借给一个Isolate
    pub isolate: OwnedIsolate,
}

#[derive(Debug, Default)]
pub struct JsRuntimeParams(CreateParams);

impl JsRuntimeParams {
    pub fn new(_snapshot: Option<Vec<u8>>) -> Self {
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
        {
            // 从 GlobalState 中获取 context
            let context = JsRuntimeState::get_context(&mut isolate);
            // 获取 scope, 用于创建 v8::object
            let handle_scope = &mut HandleScope::with_context(&mut isolate, context);
            Extensions::install(handle_scope);
        }
        Self { isolate }
    }
}
