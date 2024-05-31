pub mod command;
pub mod decode;
pub mod encode;

use bytes::BytesMut;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode: Sized {
    fn decode(buf: &BytesMut) -> Result<Self, RespError>;
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
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleString(String);
pub struct SimpleError(String);
pub struct RespNullBulkString;
pub struct BulkString(Vec<u8>);
pub struct RespArray(Vec<RespFrame>);
pub struct RespNull;
pub struct RespNullArray;
pub struct RespMap(HashMap<String, RespFrame>);
pub struct RespSet(HashSet<RespFrame>);

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
}
