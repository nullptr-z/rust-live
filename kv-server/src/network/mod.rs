mod frame;
mod tls;

use bytes::BytesMut;
pub use frame::*;
use prost::Message;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tracing::info;

use crate::{CommandRequest, CommandResponse, KvError, MemoryDB, Service, Storage};

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
