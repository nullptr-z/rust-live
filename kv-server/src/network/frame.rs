use crate::{CommandRequest, CommandResponse, KvError};
use bytes::{Buf, BufMut, BytesMut};
use flate2::{read::GzDecoder, write, Compression};
use prost::Message;
use std::io::{Read, Write};
use tracing::debug;

/// 长度标识位，表示占用 4 字节
pub const LEN_LEN: usize = 4;
/// 长度标识位,最高位代表是否压缩
const COMPRESSION_BIT: usize = 1 << ((LEN_LEN * 8) - 1);
/// 长度占用 31bit，所以 frame 大小是 2G
const MAX_FRAME: usize = (!COMPRESSION_BIT as u32) as _;
/// 如果 payload 超过 1436字节，就做压缩；1500MTU - IP头20Byte - TPC头20Byte - TCP.option(预留20Byte) - 长度标识位4Byte = 1436
const COMPRESSION_LIMIT: usize = 1436;

pub trait FrameCoder
where
    Self: Message + Sized + Default,
{
    // encode Message into a Frame, and write it into buf
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

            // 起始位置移动 LEN_LEN 获取数据部分; 分隔后，buf=长度，payload=数据
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

            // 写入压缩后长度； | COMPRESSION_BIT防止超出，不过压缩后不应该长度更长
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

    // decode Frame into Message
    fn frame_decode(buf: &mut BytesMut) -> Result<Self, KvError> {
        todo!()
    }
}

impl FrameCoder for CommandRequest {}
impl FrameCoder for CommandResponse {}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;

    use crate::{CommandRequest, CommandResponse, FrameCoder, Value};

    use super::COMPRESSION_LIMIT;

    #[test]
    fn command_request_encode_decode_should_wrok() {
        let mut buf = BytesMut::new();
        let cmd = CommandRequest::new_hset("table1", "key1", "value1".into());
        cmd.frame_encode(&mut buf).unwrap();

        // 不需要压缩
        assert_eq!(is_compressed(&buf), false);

        // let cmd1 = comm
    }

    #[test]
    fn command_response_compressed_encode_decode_should_work() {
        let mut buf = BytesMut::new();

        let value: Value = vec![0u8; COMPRESSION_LIMIT].as_slice().try_into().unwrap();
        let res: CommandResponse = value.into();
        res.frame_encode(&mut buf).unwrap();

        // 需要压缩
        assert_eq!(is_compressed(&buf), true);
        // let cmd = CommandRequest::new_hset("table1", "key1", value);
    }

    fn is_compressed(data: &[u8]) -> bool {
        if let &[v] = &data[..1] {
            v >> 7 == 1
        } else {
            false
        }
    }
}
