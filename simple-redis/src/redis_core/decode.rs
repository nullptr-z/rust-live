use bytes::BytesMut;

use super::{RespDecode, RespFrame};

impl RespDecode for BytesMut {
    fn decode(buf: Self) -> Result<RespFrame, String> {
        todo!()
    }
}
