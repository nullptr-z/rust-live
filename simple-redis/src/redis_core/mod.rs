pub mod command;
pub mod decode;
pub mod encode;

use bytes::{Buf, BytesMut};
use std::collections::BTreeMap;
use thiserror::Error;

const CRLF_LEN: usize = 2;

pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode: Sized {
    const PREFIX: &'static str = "";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;
    // fn expect_prefix(buf: &mut BytesMut) -> Result<(), RespError>;
    fn expect_length(buf: &[u8]) -> Result<usize, RespError>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RespFrame {
    Integer(i64),
    SimpleString(SimpleString),
    Error(SimpleError),
    BulkString(BulkString),
    NullBulkString(RespNullBulkString),
    Array(RespArray),
    Null(RespNull),
    NullArray(RespNullArray),
    Boolean(bool),
    Double(f64),
    Map(RespMap),
    Set(RespSet),
}

// 为了区分 SimpleString 和 SimpleError，实际上他们都是 String
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SimpleString(String);
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SimpleError(String);
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespNullBulkString;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BulkString(Vec<u8>);
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespArray(Vec<RespFrame>);
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespNull;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespNullArray;
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespMap(BTreeMap<String, RespFrame>);
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct RespSet(Vec<RespFrame>);

#[derive(Debug, Error, PartialEq)]
pub enum RespError {
    #[error("invalid frame: {0}")]
    InvalidFrame(String),
    #[error("invalid frame type: {0}")]
    InvalidFrameType(String),
    #[error("invalid frame length: {0}")]
    InvalidFrameLength(isize),
    #[error("frame is not complete")]
    NotComplete,
    #[error("Parse int error: {0}")]
    parseIntError(#[from] std::num::ParseIntError), // 从 std::num::ParseIntError 转换到自定义的 parseIntError
    #[error("Parse float error: {0}")]
    parseFloatError(#[from] std::num::ParseFloatError), // 从 std::num::ParseFloatError 转换到自定义的 parseFloatError
}

impl SimpleString {
    fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

fn extract_fixed_data(
    buf: &mut BytesMut,
    extract: &str,
    extract_type: &str,
) -> Result<(), RespError> {
    if buf.len() < extract.len() {
        return Err(RespError::NotComplete);
    }

    if !buf.starts_with(extract.as_bytes()) {
        return Err(RespError::InvalidFrameType(format!(
            "expect: {}, but got: {:?}",
            extract_type, buf
        )));
    }

    buf.advance(extract.len());

    Ok(())
}

fn extract_simple_frame_data(buf: &mut BytesMut, prefix: &str) -> Result<String, RespError> {
    let len = buf.len();
    // length 至少大与 3
    if len < 3 {
        return Err(RespError::NotComplete);
    }
    if !buf.starts_with(prefix.as_bytes()) {
        return Err(RespError::InvalidFrameType(format!(
            "Simple Frame need char '{}' start",
            prefix
        )));
    }

    // end is "\r\n"
    let end = match find_nth_crlf(&buf, 1) {
        Some(end) => end,
        None => return Err(RespError::NotComplete),
    };

    let data = buf.split_to(end + 2);
    let content = String::from_utf8_lossy(&data[1..end]);

    Ok(content.to_string())
}

fn find_nth_crlf(buf: &[u8], nth: usize) -> Option<usize> {
    let mut count: usize = 0;
    for i in 0..buf.len() - 1 {
        if buf[i] == b'\r' && buf[i + 1] == b'\n' {
            count += 1;
            if count == nth {
                return Some(i);
            }
        }
    }

    None
}

fn parse_length(buf: &mut BytesMut, prefix: &str) -> Result<(usize, usize), RespError> {
    let end = match find_nth_crlf(&buf, 1) {
        Some(end) => end,
        None => return Err(RespError::NotComplete),
    };
    let content = String::from_utf8_lossy(&buf[prefix.len()..end]);
    Ok((end, content.parse()?))
}

fn calc_total_length(buf: &[u8], len: usize, prefix: &str) -> Result<usize, RespError> {
    let data = &buf[len + CRLF_LEN..];
    match prefix {
        "*" | "~" => find_nth_crlf(data, len)
            .map(|end| end + CRLF_LEN + len)
            .ok_or(RespError::NotComplete),
        "%" => find_nth_crlf(data, len * 2)
            .map(|end| end + CRLF_LEN + len)
            .ok_or(RespError::NotComplete),
        _ => Ok(len + CRLF_LEN),
    }
}

impl RespArray {
    fn new(v: impl Into<Vec<RespFrame>>) -> Self {
        RespArray(v.into())
    }
}

impl From<&[u8]> for SimpleString {
    fn from(s: &[u8]) -> Self {
        s.into()
    }
}

// impl From<&str> for SimpleString {
//     fn from(s: &str) -> Self {
//         SimpleString::new(s)
//     }
// }

impl From<SimpleString> for RespFrame {
    fn from(s: SimpleString) -> Self {
        RespFrame::SimpleString(s)
    }
}

impl From<&str> for RespFrame {
    fn from(s: &str) -> Self {
        RespFrame::BulkString(s.into())
    }
}

impl From<&[u8]> for RespFrame {
    fn from(s: &[u8]) -> Self {
        s.into()
    }
}

impl From<Vec<RespFrame>> for RespArray {
    fn from(s: Vec<RespFrame>) -> Self {
        RespArray::new(s)
    }
}

impl From<BulkString> for RespFrame {
    fn from(s: BulkString) -> Self {
        RespFrame::BulkString(s)
    }
}

impl From<&str> for BulkString {
    fn from(s: &str) -> Self {
        BulkString(s.as_bytes().to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for RespFrame {
    fn from(value: &[u8; N]) -> Self {
        BulkString(value.to_vec()).into()
    }
}
