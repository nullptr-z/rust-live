## extensions
注册 Rust 函数，供JavaScript调用

## event_loop
基于poll_event_loop封装····································································································· ，可以在单线程中实现异步编程。

只能在单线程中使用 event loop ，不能在多线程中使用 event loop 。

v8引擎的 Isolate 有一个 event loop，可以用来执行异步任务，但是这个 event loop 是单线程的，所以异步任务也是单线程的，这个 event loop 与 nodejs 的 event loop 是不同的，nodejs 的 event loop 是多线程的，所以异步任务也是多线程的。

由于单线程这个限制，要在 tokio 中使用 event loop ，需要使用 tokio::task::current_thread 或者 tokio::task::spawn_blocking ，才可以调用 event loop。

# JsRuntime
只能在创建它的线程中使用，不能在其他线程中使用。

## load_main_module
加载JavaScript 文件到内存中，它会被编译成一个 JavaScript 模块，接着实例化它，我们得到一个它的 module ID，然后在 Rust 中通过 mod_evaluate 执行这个模块。
