

在webasockt中，我们需要使用到一个库，这个库是axum-ws，这个库是axum的一个扩展，用于处理websocket的请求。


## demo 使用指南
主要参考`src/lib.rs`文件，使用`axum_ws::ws`函数来处理websocket的请求，这个函数接收一个`MsgData`类型的消息，这个消息是我们自定义的，我们可以在这个消息中定义我们需要的消息格式，
```rs
// 消息类型
pub enum MsgData {
    Json,
    Leave,
    Message(String),
}
```


## 强关联Crates
* axum

## 用到的Crates
* dashmap
* Arc
* RwLock

## 关键词
* websocket
* axum-ws
* 消息
* 通信


## 技巧
在做一个 WebSocket 服务的时候，我们首先需要考虑定义一个消息的格式，这个消息格式可以是任意的，但是我们需要在客户端和服务端都能够解析。例如：
```rs
enum Message {
    Text(String),
    Binary(Vec<u8>),
}
``
这个消息支持文本和二进制两种格式，我们可以在客户端和服务端都使用这个消息格式，这样就可以在客户端和服务端之间传递消息了。
