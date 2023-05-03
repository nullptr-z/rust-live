

deno runtime/core 的 worker 和 Chrome v8的 Isolate 单线程异步的,只能固定在一个线程下运行，运行在多线程环境场景问题：

> tokio多线程运行时(multi threads runtime),采用 Work stealing策略, 可能会把一个运行了一半的 Future 任务挂起，唤醒时可能切换到另一个线程上运行，这样就会导致一个任务在多个线程上运行。这是不允许的。

解决方法：
1. 在当前线 current thread上程创建 blocking来执行异步任务。当前线程不能是 multi threads runtime;
2. 或者是把整个 worker 都转移到一个固定新的线程下运行， std::thread::spawn(move ||{ ... })，这样就不会出现多线程运行的问题。 这种方案适合在当前线程已经是 multi threads runtime的情况。

