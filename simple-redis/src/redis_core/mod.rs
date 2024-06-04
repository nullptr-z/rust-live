pub mod command;
pub mod decode;
pub mod encode;

use bytes::{Buf, BytesMut};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

const CRLF_LEN: usize = 2;

pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode: Sized {
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;
}

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleString(String);
pub struct SimpleError(String);
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RespNullBulkString;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BulkString(Vec<u8>);
pub struct RespArray(Vec<RespFrame>);
pub struct RespNull;
pub struct RespNullArray;
pub struct RespMap(HashMap<String, RespFrame>);
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
    extract: &[u8],
    extract_type: &str,
) -> Result<(), RespError> {
    if buf.len() < extract.len() {
        return Err(RespError::NotComplete);
    }

    if !buf.starts_with(extract) {
        return Err(RespError::InvalidFrameType(format!(
            "expect: {}, but got: {:?}",
            extract_type, buf
        )));
    }

    buf.advance(extract.len());

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
    let end: usize = find_nth_crlf(&buf, 1)?;

    let data = buf.split_to(end + 2);
    let content = String::from_utf8_lossy(&data[1..end]);

    Ok(content.to_string())
}

fn find_nth_crlf(buf: &BytesMut, nth: usize) -> Result<usize, RespError> {
    let mut count: usize = 0;
    for i in 0..buf.len() - 1 {
        if buf[i] == b'\r' && buf[i + 1] == b'\n' {
            count += 1;
            if count == nth {
                return Ok(i);
            }
        }
    }

    Err(RespError::NotComplete)
}
fn parse_length(buf: &mut BytesMut, prefix: &[u8]) -> Result<(usize, usize), RespError> {
    let end = find_nth_crlf(buf, 1)?;
    let content = String::from_utf8_lossy(&buf[prefix.len()..end]);
    Ok((end, content.parse()?))
}
