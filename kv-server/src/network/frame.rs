use crate::{CommandRequest, CommandResponse, KvError};
use bytes::{Buf, BufMut, BytesMut};
use flate2::{read, write, Compression};
use prost::Message;
use std::io::{Read, Write};
use tokio::io::AsyncReadExt;
use tracing::debug;

/// 长度标识位，表示占用 4 字节
pub const LEN_LEN: usize = 4;
/// 长度标识位,最高位代表是否压缩
const COMPRESSION_BIT: usize = 1 << ((LEN_LEN * 8) - 1);
/// 长度占用 31bit，所以 frame 大小是 2G
const MAX_FRAME: usize = (!COMPRESSION_BIT as u32) as _;
/// 如果 payload 超过 1436字节，就做压缩；1500MTU - IP头20Byte - TPC头20Byte - TCP.option(预留20Byte) - 长度标识位4Byte = 1436
const COMPRESSION_LIMIT: usize = 1436;

impl FrameCoder for CommandRequest {}
impl FrameCoder for CommandResponse {}

pub trait FrameCoder
where
    Self: Message + Sized + Default,
{
    /// encode Message into a Frame, and write it into buf \
    /// 把 Message 封包成一个 Frame, 存入 buf
    fn frame_encode(&self, buf: &mut BytesMut) -> Result<(), KvError> {
        let size = self.encoded_len();
        // 如果数据大小大于帧的大小限制，抛出异常
        if size >= MAX_FRAME {
            return Err(KvError::FrameError(size.to_string()));
        }

        buf.put_u32(size as _);

        // 数据大小达到的压缩长度进行压缩
        if size >= COMPRESSION_LIMIT {
            // 数据缓冲区
            let mut buf1 = Vec::with_capacity(size);
            // 将数据字节流写入buf1
            self.encode(&mut buf1)?;

            // 起始位置移动 LEN_LEN, 这里数据起始字节位的位置;
            let payload = buf.split_off(LEN_LEN);
            // 清除缓冲区
            buf.clear();

            // 开始压缩, 将payload的写入器，传给压缩器作为缓冲区
            let mut encoder = write::GzEncoder::new(payload.writer(), Compression::default());
            // 指定需要压缩的数据字节流
            encoder.write_all(&buf1[..])?;

            // 执行压缩，返回压缩后的数据
            let payload = encoder.finish()?.into_inner();
            debug!("Encode a frame: size {}({})", size, payload.len());

            // 写入压缩后长度； 置位压缩标识
            buf.put_u32((payload.len() | COMPRESSION_BIT) as _);
            // 写入压缩后的数据流
            buf.unsplit(payload);

            Ok(())
        } else {
            // 不压缩直接写入 buf
            self.encode(buf)?;

            Ok(())
        }
    }

    /// decode Frame into Message
    /// 将 buf 中保存的 Frame 解码成 Message
    fn frame_decode(buf: &mut BytesMut) -> Result<Self, KvError> {
        // 获取长度标识符, get_u32会将指针移动 4 字节
        let header = buf.get_u32() as usize;
        //  size: 数据的长度; compressed: frame是否压缩
        let (size, compressed) = decode_header(header);

        // 如果解压过，这里要解压缩
        if compressed {
            // 设置缓冲区
            let mut buf1 = Vec::with_capacity(LEN_LEN + size);
            // 设置被压缩数据源，准备解压
            let mut decoder = read::GzDecoder::new(&buf[..size]);
            // 将解压后数据写入缓冲区
            decoder.read_to_end(&mut buf1)?;
            // buf.advance(size);

            // decode
            Ok(Self::decode(buf1.as_slice())?)
        } else {
            let decode = Self::decode(&buf[..size]);
            // buf.advance(size);

            Ok(decode?)
        }
    }
}

fn decode_header(header: usize) -> (usize, bool) {
    let size = header & MAX_FRAME;
    let compressed = header & COMPRESSION_BIT == COMPRESSION_BIT;
    (size, compressed)
}

/**
 * 客户端接受一个响应，服务端接受一个请求时都需要用到
 * 从 Stream 中读取数据流 , 保存到 buf
 * @param stream TCP stream
 * @param buf 帧的缓冲区frame buffer zone
 * 这是一个通用方法 Stream 可以是 TPC,HTTP,WebSocket
 * [参考](../../README.md#AsyncReadExt )
 */
pub async fn read_frame<S>(stream: &mut S, buf: &mut BytesMut) -> Result<(), KvError>
where
    // AsyncRead: 用于异步读取, 实现了 AsyncRead 就自动实现了AsyncReadExt，AsyncReadExt有大量用作读取的辅助方法
    //  [ref`参考](../../README.md#AsyncReadExt )
    S: tokio::io::AsyncRead + Unpin + Send,
{
    let header = stream.read_u32().await? as usize;
    let (size, _compressed) = decode_header(header);
    // 至少要有一个 frame 的大小: 头部 4 字节 + frame大小
    buf.reserve(LEN_LEN + size);
    // 拼装头部4字节
    buf.put_u32(header as _);
    // 推进光标
    // reserve这段内存，然后再这个段范围内写入数据，这里是安全使用的
    unsafe { buf.advance_mut(size) };
    stream.read_exact(&mut buf[LEN_LEN..]).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use tokio::io::AsyncRead;

    use crate::{read_frame, CommandRequest, CommandResponse, FrameCoder, Value};

    use super::COMPRESSION_LIMIT;

    struct DummyStream {
        buf: BytesMut,
    }

    impl AsyncRead for DummyStream {
        /// DummyStream 中保存着协议数据流：TCPStream、SoketStream
        /// 将（Self.buf）DummyStream 中的数据切割并写入到提供的缓冲`buf`区中; 这个过程是异步的
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            // 缓冲区能装多少数据
            let size = buf.capacity();
            // 取出相同大小`size`的数据 ot data
            let data = self.get_mut().buf.split_to(size);
            // 把取出来的数据写入缓冲区
            buf.put_slice(&data);

            std::task::Poll::Ready(Ok(()))
        }
    }

    #[test]
    fn command_request_encode_decode_should_wrok() {
        let mut buf = BytesMut::new();
        let cmd = CommandRequest::new_hset("table1", "key1", "value1".into());
        cmd.frame_encode(&mut buf).unwrap();

        // 不需要压缩
        assert_eq!(is_compressed(&buf), false);

        let cmd1 = CommandRequest::frame_decode(&mut buf).unwrap();

        assert_eq!(cmd1, cmd);
    }

    #[test]
    fn command_response_encode_decode_should_work() {
        let mut buf = BytesMut::new();
        /// 从数据生成 response
        let values: Vec<Value> = vec![1.into(), "hello".into(), b"data".into()];
        let res: CommandResponse = values.into();

        res.frame_encode(&mut buf).unwrap();
        // 不需要压缩
        assert_eq!(is_compressed(&buf), false);

        let res1 = CommandResponse::frame_decode(&mut buf).unwrap();

        assert_eq!(res, res1);
    }

    #[test]
    fn command_response_compressed_encode_decode_should_work() {
        let mut buf = BytesMut::new();

        let value: Value = vec![0u8; COMPRESSION_LIMIT].as_slice().try_into().unwrap();
        let res: CommandResponse = value.into();
        res.frame_encode(&mut buf).unwrap();

        // 需要压缩
        assert_eq!(is_compressed(&buf), true);

        let res_decode = CommandResponse::frame_decode(&mut buf).unwrap();

        assert_eq!(res, res_decode)
    }

    /// 测试read_frame，模拟从 Stream 读取数据流。从 Socket 读数据的流程
    #[tokio::test]
    async fn read_frame_should_work() {
        // 封包一个 cmd
        let mut buf = BytesMut::new();
        let cmd = CommandRequest::new_hset("table", "key", "value".into());
        cmd.frame_encode(&mut buf).unwrap();
        let mut stream = DummyStream { buf };

        // 模拟从 Stream 读取数据
        let mut stream_buf = BytesMut::new();
        read_frame(&mut stream, &mut stream_buf).await.unwrap();
        let cmd1 = CommandRequest::frame_decode(&mut stream_buf).unwrap();

        assert_eq!(cmd, cmd1);
    }

    fn is_compressed(data: &[u8]) -> bool {
        if let &[v] = &data[..1] {
            v >> 7 == 1
        } else {
            false
        }
    }
}
