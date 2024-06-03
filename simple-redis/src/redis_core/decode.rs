use std::borrow::Cow;

use bytes::{Buf, BufMut, BytesMut};
use tracing::info;

use super::*;

impl RespDecode for RespFrame {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let mut buf_iter = buf.iter().peekable();
        match buf_iter.peek() {
            Some(b'+') => {}
            None => todo!(),
            _ => todo!(),
        }

        Ok(RespFrame::Integer(0))
    }
}

impl RespDecode for SimpleString {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let content = extract_simple_frame_data(buf, b"+")?;
        info!("got a SimpleString: {}", content);

        Ok(SimpleString(content.to_string()))
    }
}

impl RespDecode for SimpleError {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let content = extract_simple_frame_data(buf, b"-")?;
        info!("got a SimpleError: {}", content);

        Ok(SimpleError(content.to_string()))
    }
}

impl RespDecode for i64 {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let content = extract_simple_frame_data(buf, b":")?;
        info!("got a SimpleError: {}", content);

        Ok(content.parse()?)
    }
}

impl RespDecode for bool {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        match extract_fixed_data(buf, b"#t\r\n", "Bool") {
            Ok(_) => Ok(true),
            Err(_) => match extract_fixed_data(buf, b"#f\r\n", "Bool") {
                Ok(_) => Ok(false),
                Err(e) => Err(e),
            },
        }
    }
}

fn extract_fixed_data(buf: &mut BytesMut, prefix: &[u8], types: &str) -> Result<(), RespError> {
    if buf.len() < prefix.len() {
        return Err(RespError::NotComplete);
    }

    if !buf.starts_with(prefix) {
        return Err(RespError::InvalidFrameType(format!(
            "expect: {}, but got: {:?}",
            types, buf
        )));
    }

    buf.advance(prefix.len());

    Ok(())
}

fn extract_simple_frame_data(buf: &mut BytesMut, prefix: &[u8]) -> Result<String, RespError> {
    let len = buf.len();
    // length 至少大与 3
    if len < 3 {
        return Err(RespError::NotComplete);
    }
    if !buf.starts_with(prefix) {
        return Err(RespError::InvalidFrameType(format!(
            "Simple Frame need char '{:?}' start",
            prefix
        )));
    }

    // end is "\r\n"
    let mut end: usize = 0;
    for i in 0..buf.len() - 1 {
        if buf[i] == b'\r' && buf[i + 1] == b'\n' {
            end = i;
            break;
        }
    }
    if end == 0 {
        return Err(RespError::NotComplete);
    }
    let data = buf.split_to(end + 2);
    let content = String::from_utf8_lossy(&data[1..end]);

    Ok(content.to_string())
}

#[test]
fn test_decode() {
    let mut buf = BytesMut::new();
    buf.extend(b"+hi\r\n");
    let frame = SimpleString::decode(&mut buf);
    assert_eq!(frame.unwrap(), SimpleString("hi".into()));

    buf.extend_from_slice(b"+redis\r");
    let frame = SimpleString::decode(&mut buf);
    assert_eq!(frame.unwrap_err(), RespError::NotComplete);

    buf.put_u8(b'\n');
    let frame = SimpleString::decode(&mut buf);
    assert_eq!(frame.unwrap(), SimpleString::new("redis"));
}
