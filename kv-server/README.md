# core idea:

- 从 TCPStream 读取 Frame
- Frame 头部 Header 4 字节包换了数据字节流的长度
- 其中 Header 首字节位，标识数据流部分是否压缩过

## why

TcpStream：TCP 协议的报文，可以通过自定义封包/解析帧(Frame)，实现自己的上层协议，关于帧长度可以直接计算 buf length，帧类型可以使用 protobuf 来完成

## AsyncReadExt

`tokio::io::util::async_read_ext::AsyncReadExt` 有大量用作读取的异步方法
