pub mod abi;

use abi::{command_request::RequestData, *};
use bytes::Bytes;
use hyper::StatusCode;

use crate::error::KvError;

impl CommandRequest {
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        RequestData::Hset(Hset {
            table: table.into(),
            pair: Some(Kvpair::new(key, value)),
        })
        .into()
    }

    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        RequestData::Hget(Hget {
            table: table.into(),
            key: key.into(),
        })
        .into()
    }

    pub fn new_hdel(table: impl Into<String>, key: impl Into<String>) -> Self {
        RequestData::Hdel(Hdel {
            table: table.into(),
            key: key.into(),
        })
        .into()
    }

    pub fn new_hgetall(table: impl Into<String>) -> Self {
        RequestData::Hgetall(Hgetall {
            table: table.into(),
        })
        .into()
    }

    pub fn new_hmget(table: impl Into<String>, key: impl Into<String>) -> Self {
        RequestData::Hmget(Hmget {
            table: table.into(),
            keys: vec![key.into()],
        })
        .into()
    }
}

impl Kvpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

impl From<RequestData> for CommandRequest {
    fn from(value: RequestData) -> Self {
        Self {
            request_data: Some(value),
        }
    }
}

impl From<Hset> for RequestData {
    fn from(value: Hset) -> Self {
        RequestData::Hset(value)
    }
}

impl From<Hget> for RequestData {
    fn from(value: Hget) -> Self {
        RequestData::Hget(value)
    }
}

impl From<Hdel> for RequestData {
    fn from(value: Hdel) -> Self {
        RequestData::Hdel(value)
    }
}

impl From<Value> for CommandResponse {
    fn from(value: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![value],
            ..Default::default()
        }
    }
}

impl From<KvError> for CommandResponse {
    fn from(value: KvError) -> Self {
        let mut result = Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: value.to_string(),
            values: vec![],
            pairs: vec![],
        };

        match value {
            KvError::NotFound(_, _) => result.status = StatusCode::NOT_FOUND.as_u16() as _,
            KvError::InvalidCommand(_) => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
            _ => {}
        }

        result
    }
}

impl From<(String, Value)> for Kvpair {
    fn from(kv: (String, Value)) -> Self {
        Kvpair {
            key: kv.0,
            value: Some(kv.1),
        }
    }
}

impl From<Vec<Kvpair>> for CommandResponse {
    fn from(value: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            pairs: value,
            ..Default::default()
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(value)),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self {
            value: Some(value::Value::String(value)),
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self {
            value: Some(value::Value::String(value.into())),
        }
    }
}

impl From<&[u8]> for Value {
    fn from(value: &[u8]) -> Self {
        let str = String::from_utf8_lossy(value);
        Self {
            value: Some(value::Value::String(str.into())),
        }
    }
}

impl From<Bytes> for Value {
    fn from(value: Bytes) -> Self {
        Self {
            value: Some(value::Value::Binary(value)),
        }
    }
}

impl<const N: usize> From<&[u8; N]> for Value {
    fn from(buf: &[u8; N]) -> Self {
        Bytes::copy_from_slice(&buf[..]).into()
    }
}
