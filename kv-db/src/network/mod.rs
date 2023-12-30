mod frame;

use crate::error::KvError;
use bytes::BytesMut;
use prost::Message;

pub trait Frame
where
    Self: Sized + Message + Default,
{
    // 将Message封包(encode)成Frame
    fn encode_frame(&self, buf: &mut BytesMut) -> Result<(), KvError>;
    // 将一个Frame解包(decode)成Message
    fn decode_frame(buf: &mut BytesMut) -> Result<Self, KvError>;
}
