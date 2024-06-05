#[allow(unused)]
use bytes::{Buf, BufMut, BytesMut};
use tracing::info;

use super::*;

impl RespDecode for RespFrame {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let mut buf_iter = buf.iter().peekable();
        let resp = match buf_iter.peek() {
            Some(b':') => Ok(RespFrame::Integer(i64::decode(buf)?)),
            Some(b'+') => Ok(RespFrame::SimpleString(SimpleString::decode(buf)?)),
            Some(b'-') => Ok(RespFrame::Error(SimpleError::decode(buf)?)),
            Some(b',') => Ok(RespFrame::Double(f64::decode(buf)?)),
            Some(b'_') => Ok(RespFrame::Null(RespNull::decode(buf)?)),
            Some(b'%') => Ok(RespFrame::Map(RespMap::decode(buf)?)),
            Some(b'~') => Ok(RespFrame::Set(RespSet::decode(buf)?)),
            Some(b'#') => Ok(RespFrame::Boolean(bool::decode(buf)?)),
            Some(b'*') => match RespNullArray::decode(buf) {
                Ok(r) => Ok(RespFrame::NullArray(r)),
                Err(RespError::NotComplete) => Err(RespError::NotComplete),
                Err(_) => Ok(RespFrame::Array(RespArray::decode(buf)?)),
            },
            Some(b'$') => match RespNullBulkString::decode(buf) {
                Ok(r) => Ok(RespFrame::NullBulkString(r)),
                Err(RespError::NotComplete) => Err(RespError::NotComplete),
                Err(_) => Ok(RespFrame::BulkString(BulkString::decode(buf)?)),
            },
            None => Ok(RespFrame::Null(RespNull::decode(buf)?)),
            _ => Err(RespError::NotComplete),
        };

        resp
    }
}

impl RespDecode for RespNull {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        extract_fixed_data(buf, "_\r\n", "Null")?;

        Ok(RespNull)
    }
}

impl RespDecode for SimpleString {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let content = extract_simple_frame_data(buf, "+")?;
        info!("got a SimpleString: {}", content);

        Ok(SimpleString(content.to_string()))
    }
}

impl RespDecode for BulkString {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let (end, len) = parse_length(buf, "$")?;
        let remained = &buf[end + CRLF_LEN..];
        if remained.len() < len + CRLF_LEN {
            return Err(RespError::NotComplete);
        }

        buf.advance(end + CRLF_LEN);
        let data = buf.split_to(len + CRLF_LEN);

        Ok(BulkString(data[..len].to_vec()))
    }
}

impl RespDecode for RespNullBulkString {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        extract_fixed_data(buf, "$-1\r\n", "NullBulkString")?;

        Ok(RespNullBulkString)
    }
}

impl RespDecode for SimpleError {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let content = extract_simple_frame_data(buf, "-")?;
        info!("got a SimpleError: {}", content);

        Ok(SimpleError(content.to_string()))
    }
}

impl RespDecode for bool {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        match extract_fixed_data(buf, "#t\r\n", "Bool") {
            Ok(_) => Ok(true),
            Err(_) => match extract_fixed_data(buf, "#f\r\n", "Bool") {
                Ok(_) => Ok(false),
                Err(e) => Err(e),
            },
        }
    }
}

impl RespDecode for i64 {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let content = extract_simple_frame_data(buf, ":")?;
        info!("got a RespDecode: {}", content);

        Ok(content.parse()?)
    }
}

impl RespDecode for f64 {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let content = extract_simple_frame_data(buf, ",")?;
        info!("got a SimpleError: {}", content);

        Ok(content.parse()?)
    }
}

impl RespDecode for RespArray {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let content = extract_simple_frame_data(buf, "*")?;
        let len = content.parse::<i64>()? as usize;

        let mut frames: Vec<RespFrame> = Vec::with_capacity(len);
        for _ in 0..len {
            let next_frame = RespFrame::decode(buf)?;
            frames.push(next_frame)
        }

        Ok(RespArray(frames))
    }
}

impl RespDecode for RespNullArray {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        extract_fixed_data(buf, "*-1\r\n", "NullArray")?;

        Ok(RespNullArray)
    }
}

impl RespDecode for RespMap {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let content = extract_simple_frame_data(buf, "%")?;
        let len = content.parse::<i64>()? as usize;

        let mut frames: BTreeMap<String, RespFrame> = BTreeMap::new();
        for _ in 0..len {
            let key = SimpleString::decode(buf)?;
            let value = RespFrame::decode(buf)?;
            frames.insert(key.0, value);
        }

        Ok(RespMap(frames))
    }
}

impl RespDecode for RespSet {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let content = extract_simple_frame_data(buf, "~")?;
        let len = content.parse::<i64>()? as usize;

        let mut frames = Vec::with_capacity(len);
        for _ in 0..len {
            let value = RespFrame::decode(buf)?;
            frames.push(value);
        }

        Ok(RespSet(frames))
    }
}

mod test_decode {
    use crate::redis_core::*;
    use bytes::{BufMut, BytesMut};

    #[test]
    fn test_simple() {
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

    #[test]
    fn test_float() {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b",100.123\r\n");
        let frame = f64::decode(&mut buf);
        assert_eq!(frame.unwrap(), 100.123);

        buf.extend_from_slice(b",+100.123456e-9\r\n");
        let frame = f64::decode(&mut buf);
        assert_eq!(frame.unwrap(), 100.123456e-9)
    }

    #[test]
    fn test_bulk_string() {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"$6\r\nfoobar\r\n");
        let frame = BulkString::decode(&mut buf);
        assert_eq!(frame.unwrap(), BulkString("foobar".as_bytes().to_vec()));

        buf.extend_from_slice(b"$-1\r\n");
        let frame = RespNullBulkString::decode(&mut buf);
        assert_eq!(frame.unwrap(), RespNullBulkString);
    }

    #[test]
    fn test_set() {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"~2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n");
        let frame = RespSet::decode(&mut buf);
        assert_eq!(
            frame.unwrap(),
            RespSet(vec![
                BulkString(b"foo".to_vec()).into(),
                BulkString(b"bar".to_vec()).into()
            ])
        );
    }

    #[test]
    fn test_map() {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"%2\r\n+hello\r\n$5\r\nworld\r\n+foo\r\n$3\r\nbar\r\n");
        let frame = RespMap::decode(&mut buf);
        assert_eq!(
            frame.unwrap(),
            RespMap(BTreeMap::from([
                (
                    "hello".to_string(),
                    BulkString("world".as_bytes().to_vec()).into()
                ),
                (
                    "foo".to_string(),
                    BulkString("bar".as_bytes().to_vec()).into()
                )
            ]))
        );
    }
}
