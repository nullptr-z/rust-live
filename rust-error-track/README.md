[奇奇怪怪的错误](https://github.com/nullptr-z/Study/tree/master/Rust/note/奇奇怪怪的错误.md)

> `error[E0531]`: cannot find tuple struct or tuple variant `OK` in this scope
``错误[E0531]：在此范围内找不到元组结构或元组变体“OK”
--给一个返回值不是 Result 的类型使用了 Ok 包起来

> `error[E0599]`: no method named `funcName` found for Struct/Enum `Struct` in the current scope
`` 错误[E0599]：“Struct”结构中找不到的名为“funcName”的方法
-- 调用了一个 Result 的方法，但是这个方法不存在
-- 通常可能是调用了一个不存在的方法
-- 或者忘了unwrap()；或者忘了使用?来处理错误，或者多加了一个?

> `error[E0698]`: type inside `async` block must be known in this context
`` 错误[E0698]：此上下文中的“async”块内部的类型必须是已知的
-- 在 async 代码块中使用了一个未知的类型

> error: cannot find macro `anyhow` in this scope
`` 错误：在此范围内找不到宏“anyhow”
-- 没有引入 anyhow 库

> the trait bound `*` is not satisfied
`` 不满足 Trait 绑定限制
-- 给定参数没有实现相关 Trait
-- 使用了错误的同名create

