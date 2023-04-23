// use serde::{Deserialize, Serialize};
use v8::HandleScope;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Data {
//     pub status: usize,
//     pub message: String,
// }

fn main() {
    // init
    init();

    // create isolate
    let isolate = &mut v8::Isolate::new(Default::default());

    // create handel scope
    let handle_scope = &mut HandleScope::new(isolate);
    // create context
    let context = v8::Context::new(handle_scope);
    // enter context scope
    let context_scope = &mut v8::ContextScope::new(handle_scope, context);

    // javascript code
    let source = r#"
        function add(a, b){
            return a + b
        }
        add(1,2);
      "#;
    println!("【 result 】==>");

    // js源代码转换成v8源代码
    let source = v8::String::new(context_scope, source).unwrap();
    // compile编译v8源代码
    let script = v8::Script::compile(context_scope, source, None).unwrap();
    // 运行代码
    let result = script.run(context_scope).unwrap();
    // 将运行结果转换成json，这样比较容易得到一个我们没有定义的结构体
    let value: serde_json::Value = serde_v8::from_v8(context_scope, result).unwrap();
    println!("【 value 】==> {:?}", value);
}

fn init() {
    // query: thread pool size是做什么的？为什么要设置为0：因为不需要线程池
    // query: idle task runner是做什么的？为什么要设置为false: 因为不需要在后台运行任务
    // make_shared  为什么要用这个？因为需要一个shared_ptr, shared_ptr是一个智能指针，它会自动释放内存
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
}
