use std::ops::Deref;

use deno_runtime::{worker::WorkerOptions, BootstrapOptions};

pub struct MainWorkerOptions(WorkerOptions);

impl Deref for MainWorkerOptions {
    type Target = WorkerOptions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for MainWorkerOptions {
    fn default() -> Self {
        let bootstrap = BootstrapOptions {
            args: todo!(),
            apply_source_maps: todo!(),
            cpu_count: todo!(),
            debug_flag: todo!(),
            enable_testing_features: todo!(),
            location: todo!(),
            no_color: todo!(),
            is_tty: todo!(),
            runtime_version: todo!(),
            ts_version: todo!(),
            unstable: todo!(),
        };
        Self(WorkerOptions {
            bootstrap,
            extensions: todo!(),
            unsafely_ignore_certificate_errors: todo!(),
            root_cert_store: todo!(),
            user_agent: todo!(),
            seed: todo!(),
            module_loader: todo!(),
            create_web_worker_cb: todo!(),
            web_worker_preload_module_cb: todo!(),
            js_error_create_fn: todo!(),
            maybe_inspector_server: todo!(),
            should_break_on_first_statement: todo!(),
            get_error_class_fn: todo!(),
            origin_storage_dir: todo!(),
            blob_store: todo!(),
            broadcast_channel: todo!(),
            shared_array_buffer_store: todo!(),
            compiled_wasm_module_store: todo!(),
        })
    }
}
