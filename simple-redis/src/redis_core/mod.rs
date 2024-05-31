pub mod command;
pub mod decode;
pub mod encode;

use std::collections::{HashMap, HashSet};

pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(buf: Self) -> Result<RespFrame, String>;
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
pub struct SimpleString(String);
pub struct SimpleError(String);
pub struct RespNullBulkString;
pub struct BulkString(Vec<u8>);
pub struct RespArray(Vec<RespFrame>);
pub struct RespNull;
pub struct RespNullArray;
pub struct RespMap(HashMap<String, RespFrame>);
pub struct RespSet(HashSet<RespFrame>);
