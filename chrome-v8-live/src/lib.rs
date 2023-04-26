mod extensions;
mod state;
mod utils;

use extensions::{Extensions, EXTERNAL_REFERENCES};
use once_cell::sync::OnceCell;
use state::JsRuntimeState;
use utils::execute_script;
use v8::{CreateParams, HandleScope, Isolate, OwnedIsolate, V8};

type LocalValue<'a> = v8::Local<'a, v8::Value>;

pub struct JsRuntime {
    // OwnedIsolate 可以理解为一个Isolate的所有权，他可以借给一个Isolate
    pub isolate: OwnedIsolate,
}

#[derive(Debug, Default)]
pub struct JsRuntimeParams(CreateParams);

impl JsRuntimeParams {
    pub fn new(snapshot: Option<Vec<u8>>) -> Self {
        let mut params = CreateParams::default();
        if let Some(snapshot) = snapshot {
            params = params.snapshot_blob(snapshot);
        }
        JsRuntimeParams(params)
    }

    pub fn into_inner(self) -> CreateParams {
        self.0
    }
}

impl JsRuntime {
    pub fn init() {
        static V8_INSTANCE: OnceCell<()> = OnceCell::new();
        // query: 为什么要用OnceCell？因为只需要初始化一次
        // OnceCell 可以理解为一个只能初始化一次的变量,这里用于避免 V8::initialize 多次
        V8_INSTANCE.get_or_init(|| {
            // query: thread pool size是做什么的？为什么要设置为0：因为不需要线程池
            // query: idle task runner是做什么的？为什么要设置为false: 因为不需要在后台运行任务
            // make_shared  为什么要用这个？因为需要一个shared_ptr, shared_ptr是一个智能指针，它会自动释放内存
            let platform = v8::new_default_platform(0, false).make_shared();
            V8::initialize_platform(platform);
            V8::initialize();
        });
    }

    pub fn new(snapshot: Option<Vec<u8>>) -> Self {
        let mut params = CreateParams::default().external_references(&**EXTERNAL_REFERENCES);
        let mut initialized = false;
        if let Some(snapshot) = snapshot {
            params = params.snapshot_blob(snapshot);
            initialized = true;
        }
        let isolate = v8::Isolate::new(params);
        Self::init_isolate(isolate, initialized)
    }

    pub fn execute_script(
        &mut self,
        code: impl AsRef<str>,
        is_module: bool,
    ) -> Result<serde_json::Value, serde_json::Value> {
        let context = JsRuntimeState::get_context(&mut self.isolate);
        let handle_scope = &mut HandleScope::with_context(&mut self.isolate, context);

        match execute_script(handle_scope, code, is_module) {
            Ok(value) => Ok(serde_v8::from_v8(handle_scope, value).unwrap()),
            Err(err) => Err(serde_v8::from_v8(handle_scope, err).unwrap()),
        }
    }

    /// snapshot作用：
    /// 在 Chrome V8 引擎中，snapshot 是一种用于加快启动时间和减少内存占用的技术。它可以将一部分 V8 引擎的代码和数据事先编译好并保存到一个文件中，然后在应用程序启动时加载该文件，从而避免了在运行时重新编译和解析代码的过程，减少了启动时间和内存占用。
    /// 具体来说，snapshot 是一种序列化的数据格式，它包含了编译好的 JavaScript 代码、编译器的中间代码、静态数据和运行时的状态信息等。这些数据可以通过 V8 引擎的内置函数进行创建和序列化，生成的文件可以保存到磁盘上，供应用程序启动时加载。
    /// 当应用程序启动时，V8 引擎会首先读取 snapshot 文件，并使用其中保存的数据来初始化一部分运行时环境，例如对象、函数、内置类型等。这样可以避免在运行时重新编译和解析代码的过程，加快了应用程序的启动速度。同时，由于一些数据已经被编译为机器码，并保存在 snapshot 文件中，这也减少了应用程序的内存占用。
    pub fn create_snapshot_temp() -> Vec<u8> {
        let isolate: OwnedIsolate = Isolate::snapshot_creator(None);
        let mut runtime: JsRuntime = JsRuntime::init_isolate(isolate, false);

        let mut isolate = Isolate::snapshot_creator(Some(&EXTERNAL_REFERENCES));
        {
            let context = JsRuntimeState::get_context(&mut runtime.isolate);
            let handle_scope = &mut HandleScope::new(&mut runtime.isolate);
            // let handle_scope = &mut HandleScope::with_context(&mut runtime.isolate, context);
            let context = v8::Local::new(handle_scope, context);
            isolate.set_default_context(context);
            println!("=======================");
        }
        JsRuntimeState::drop_context(&mut runtime.isolate);
        std::mem::forget(runtime);

        match isolate.create_blob(v8::FunctionCodeHandling::Keep) {
            Some(blob) => blob.to_vec(),
            None => panic!("Failed to create snapshot"),
        }
    }

    pub fn create_snapshot() -> Vec<u8> {
        let sc = Isolate::snapshot_creator(None);
        let mut Isolate = unsafe { sc.get_ownd_isolate() };
    }

    fn init_isolate(mut isolate: OwnedIsolate, initialized: bool) -> Self {
        let state = JsRuntimeState::new(&mut isolate);
        isolate.set_slot(state);
        if !initialized {
            // 从 GlobalState 中获取 context
            let context = JsRuntimeState::get_context(&mut isolate);
            // 获取 scope, 用于创建 v8::object
            let handle_scope = &mut HandleScope::with_context(&mut isolate, context);
            Extensions::install(handle_scope);
        }
        Self { isolate }
    }
}
