

deno runtime/core 的 worker 和 Chrome v8的 Isolate 单线程异步的，运行在多线程环境场景问题：

> 1. tokio多线程运行时(multi threads runtime), 可能会把一个运行了一半的 Future 任务挂起，唤醒时可能切换到另一个线程上运行，这样就会导致一个任务在多个线程上运行。这是不允许的。

> 2.
