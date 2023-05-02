use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::Arc,
};

use deno_runtime::{
    deno_broadcast_channel::InMemoryBroadcastChannel,
    deno_core::{error::AnyError, FsModuleLoader},
    deno_web::BlobStore,
    worker::WorkerOptions,
    BootstrapOptions,
};

pub struct MainWorkerOptions(WorkerOptions);

impl Deref for MainWorkerOptions {
    type Target = WorkerOptions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MainWorkerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl MainWorkerOptions {
    /// 和Deref的区别是，这个方法会转移所有权，返回内部的WorkerOptions
    pub fn into_inner(self) -> WorkerOptions {
        self.0
    }
}

impl Default for MainWorkerOptions {
    fn default() -> Self {
        let create_web_worker_cb = Arc::new(|_| {
            panic!("在你的浏览器或运行环境中，使用js代码`typeof Worker !== 'undefined'`检查是否支持 web_worker！");
        });
        let web_worker_event_cb = Arc::new(|_| {
            panic!("在你的浏览器或运行环境中，使用js代码`typeof Worker !== 'undefined'`检查是否支持 web_worker！");
        });

        let options = WorkerOptions {
            module_loader: Rc::new(FsModuleLoader),
            extensions: vec![],
            bootstrap: BootstrapOptions::default(),
            startup_snapshot: None,
            unsafely_ignore_certificate_errors: None,
            seed: None,
            source_map_getter: None,
            format_js_error_fn: None,
            create_web_worker_cb,
            web_worker_preload_module_cb: web_worker_event_cb.clone(),
            web_worker_pre_execute_module_cb: web_worker_event_cb,
            maybe_inspector_server: None,
            should_break_on_first_statement: false,
            should_wait_for_inspector_session: false,
            npm_resolver: None,
            get_error_class_fn: Some(&get_error_class_name),
            cache_storage_dir: None,
            origin_storage_dir: None,
            blob_store: BlobStore::default(),
            broadcast_channel: InMemoryBroadcastChannel::default(),
            shared_array_buffer_store: None,
            compiled_wasm_module_store: None,
            stdio: Default::default(),
            root_cert_store: None,
        };

        Self(options)
    }
}

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}
