use prost::bytes::BytesMut;

use crate::KvError;

pub trait FrameCoder
where
    Self: Sized + Default,
{
    // encode Message to Frame
    fn frame_decode(&self, buf: &mut BytesMut) -> Result<(), KvError>;
    // decode Frame to Message
    fn frame_encode(buf: &mut BytesMut) -> Result<Self, KvError>;
}
