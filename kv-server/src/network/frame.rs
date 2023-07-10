use prost::bytes::BytesMut;

use crate::KvError;

pub trait FrameCoder
where
    Self: Sized + Default,
{
    // encode Message into a Frame, and write it into buf
    fn frame_decode(&self, buf: &mut BytesMut) -> Result<(), KvError>;
    // decode Frame into Message
    fn frame_encode(buf: &mut BytesMut) -> Result<Self, KvError>;
}
