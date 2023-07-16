mod frame;
mod tls;

use std::{marker::PhantomData, pin::Pin, task::Poll};

use bytes::BytesMut;
pub use frame::*;
use futures::{ready, FutureExt, Sink, Stream};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};
use tracing::info;

use crate::{CommandRequest, CommandResponse, KvError, Service, Storage};

pub struct ProstStream<S, In, Out> {
    // inner stream
    stream: S,
    // write cache
    wbuf: BytesMut,
    // 记录写入了多少字节
    written: usize,
    // read cache
    rbuf: BytesMut,

    _in_type: PhantomData<In>,
    _out_type: PhantomData<Out>,
}

impl<S, In, Out> ProstStream<S, In, Out>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    pub fn new(stream: S) -> Self {
        Self {
            stream,
            written: 0,
            wbuf: BytesMut::new(),
            rbuf: BytesMut::new(),
            _in_type: PhantomData,
            _out_type: PhantomData,
        }
    }
}

impl<S, Req, Res> Unpin for ProstStream<S, Req, Res> where S: Unpin {}

impl<S, In, Out> Stream for ProstStream<S, In, Out>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    In: Unpin + Send + FrameCoder,
    Out: Unpin + Send,
{
    type Item = Result<In, KvError>;

    // 从Stream中读取一个Frame，将其解码并返回（对于客户来说解码得到Response，服务端解码得到Request）
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        // 每次读取完成后 rbuf 应该清空
        assert!(self.rbuf.len() == 0);
        // 获取整个rbuf缓冲区空间, 这里使用了一些技巧，防止了重新分配缓冲区空间，还拿到了这段空间的所有权
        let mut rest = self.rbuf.split_off(0);

        // 因为poll_xxx()系列的函数是async/await的基础，处在更底层，我们不能再这里直接使用async/await
        // 这里需要将 read_frame 返回值看做一个 Future, 但是Future是一个Trait，不能直接调用他的方法
        // 通过 Box 将 fut 处理成一个Trait object, 以此调用 FutureExt 的 poll_unpin()方法;
        // Box::Pin生成Pin<Box>, 在堆上固定的内存
        // ---------------------------------------------------------
        // 从Stream 中读取 frame，保存到 rest 中
        let fut = read_frame(&mut self.stream, &mut rest);
        ready!(Box::pin(fut).poll_unpin(cx))?;

        // 将获取到的 frame 写回rbuf
        self.rbuf.unsplit(rest);

        // 解码数据
        std::task::Poll::Ready(Some(In::frame_decode(&mut self.rbuf)))
    }
}

/// 调用Send,发动Out
impl<S, In, Out> Sink<Out> for ProstStream<S, In, Out>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
    In: Unpin + Send,
    Out: Unpin + Send + FrameCoder,
{
    type Error = KvError;

    /// 做背压
    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Ready(Ok(()))
    }

    /// 封包
    fn start_send(self: std::pin::Pin<&mut Self>, item: Out) -> Result<(), Self::Error> {
        let this = self.get_mut();

        item.frame_encode(&mut this.wbuf)?;

        Ok(())
    }

    /// 将封包好的数据写入Stream（发送数据）
    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        let this = self.get_mut();

        // cx一次性可能无法全部接受 wbuf 的字节流；
        // 所需要循环多次来写入，每次写入都记录当前位置 written
        while this.written != this.wbuf.len() {
            let n = ready!(Pin::new(&mut this.stream).poll_write(cx, &this.wbuf[this.written..]))?;
            this.written += n;
        }

        // 清除 buf
        this.wbuf.clear();
        this.written = 0;

        // 调用 Stream 的 poll_flush 确保写入
        ready!(Pin::new(&mut this.stream).poll_flush(cx)?);

        Poll::Ready(Ok(()))
    }

    // 关闭 stream
    fn poll_close(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        ready!(self.as_mut().poll_flush(cx))?;

        // 关闭stream
        ready!(Pin::new(&mut self.stream).poll_shutdown(cx))?;

        Poll::Ready(Ok(()))
    }
}

pub struct PostServerStream<S, St: Storage> {
    /// Stream
    inner: S,
    service: Service<St>,
}

impl<S, St: Storage> PostServerStream<S, St>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    pub fn new(stream: S, service: Service<St>) -> Self {
        Self {
            inner: stream,
            service,
        }
    }

    /**
     * 服务端处理客户端请求主流程函数
     * ---
     * 1.解析请求
     * 2.执行请求
     * 3.返回执行的结果
     */
    pub async fn process(mut self) -> Result<(), KvError> {
        while let Ok(cmd) = self.recv().await {
            info!("God a CommandRequest: {:?}", cmd);
            // 执行请求 CommandRequest
            let resp = self.service.execute(cmd);
            self.send(resp).await.unwrap();
        }

        Ok(())
    }

    /// 封包并返回给客户端，执行完 `mgs` CommandRequest 命令的结果
    async fn send(&mut self, msg: CommandResponse) -> Result<(), KvError> {
        let mut buf = BytesMut::new();
        // 封包
        msg.frame_encode(&mut buf)?;
        // 返回一个不可变的 Bytes 对象，它会转移底层字节缓冲区的所有权
        let encoder = buf.freeze();
        // 向 Socket 文件写入数据，也就是向客户端发送数据
        self.inner.write_all(&encoder[..]).await?;

        Ok(())
    }

    /// 将来自客户端请求的Stream(保存在Self.inner中)，解析成CommandRequest
    async fn recv(&mut self) -> Result<CommandRequest, KvError> {
        let mut buf = BytesMut::new();
        let stream = &mut self.inner;
        read_frame(stream, &mut buf).await?;
        CommandRequest::frame_decode(&mut buf)
    }
}

pub struct PostClientStream<S> {
    /// Stream
    inner: S,
}

impl<S> PostClientStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin + Send,
{
    pub fn new(stream: S) -> Self {
        Self { inner: stream }
    }

    pub async fn execute(&mut self, cmd: CommandRequest) -> Result<CommandResponse, KvError> {
        self.send(cmd).await?;

        Ok(self.recv().await?)
    }

    /// 将cmd命令封包成Stream
    async fn send(&mut self, cmd: CommandRequest) -> Result<(), KvError> {
        let mut buf = BytesMut::new();

        cmd.frame_encode(&mut buf)?;
        let encoder = buf.freeze();

        self.inner.write_all(&encoder[..]).await?;

        Ok(())
    }

    async fn recv(&mut self) -> Result<CommandResponse, KvError> {
        let mut buf = BytesMut::new();
        read_frame(&mut self.inner, &mut buf).await?;
        CommandResponse::frame_decode(&mut buf)
    }
}
