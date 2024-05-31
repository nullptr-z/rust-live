use bytes::BytesMut;

use super::{RespDecode, RespError, RespFrame, SimpleString};

impl RespDecode for RespFrame {
    fn decode(buf: &BytesMut) -> Result<Self, RespError> {
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
    fn decode(buf: &BytesMut) -> Result<Self, RespError> {
        let mut buf_iter = buf.iter().peekable();
        return match buf_iter.peek() {
            Some(b'+') => {
                let mut s = String::new();
                buf_iter.next();
                while let Some(b) = buf_iter.next() {
                    if *b == b'\r' {
                        buf_iter.next();
                        break;
                    }
                    s.push(*b as char);
                }
                return Ok(SimpleString(s));
            }
            None => Err(RespError::NotComplete),
            _ => Err(RespError::NotComplete),
        };
    }
}

#[test]
fn test_decode() {
    let mut buf = BytesMut::new();
    // buf.extend(b"+hi\r\n");
    // let frame = SimpleString::decode(&mut buf);
    // assert_eq!(frame.unwrap(), SimpleString("hi".into()));

    buf.extend_from_slice(b"the world");
    println!("【 buf 】==> {:?}", buf);
    let frame1 = SimpleString::decode(&mut buf);
    println!("【 frame 】==> {:?}", frame1);
    assert_eq!(frame1.unwrap_err(), RespError::NotComplete);
}
