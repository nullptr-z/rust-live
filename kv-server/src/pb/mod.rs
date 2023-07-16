mod abi;

pub use abi::*;
use reqwest::StatusCode;

use crate::KvError;

use self::command_request::RequestData;

impl CommandRequest {
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }

    pub fn new_hget(table: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hget(Hget {
                table: table.into(),
                key: key.into(),
            })),
        }
    }

    pub fn new_hgetall(table: impl Into<String>) -> Self {
        Self {
            request_data: Some(RequestData::Hgetall(Hgetall {
                table: table.into(),
            })),
        }
    }
}

impl Kvpair {
    pub fn new(key: impl Into<String>, value: impl Into<Value>) -> Self {
        Self {
            key: key.into(),
            value: Some(value.into()),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(value::Value::String(s.into())),
        }
    }
}

impl From<i64> for Value {
    fn from(val: i64) -> Self {
        Self {
            value: Some(value::Value::Integer(val)),
        }
    }
}

impl TryFrom<&[u8]> for Value {
    type Error = KvError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let str = Value {
            value: Some(value::Value::String(
                String::from_utf8(buf.to_vec())
                    .expect(format!("Failed String::from_utf8 for value: {:?}", buf).as_str()),
            )),
        };

        Ok(str)
    }
}

impl<const N: usize> From<&[u8; N]> for Value {
    fn from(value: &[u8; N]) -> Self {
        String::from_utf8_lossy(value).as_ref().into()
    }
}

impl From<Value> for CommandResponse {
    fn from(val: Value) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            values: vec![val],
            ..Default::default()
        }
    }
}
impl From<Vec<Value>> for CommandResponse {
    fn from(vals: Vec<Value>) -> Self {
        let mut values = vec![];
        for val in vals {
            values.push(val)
        }

        Self {
            status: StatusCode::OK.as_u16() as _,
            values,
            ..Default::default()
        }
    }
}

impl From<Vec<Kvpair>> for CommandResponse {
    fn from(v: Vec<Kvpair>) -> Self {
        Self {
            status: StatusCode::OK.as_u16() as _,
            pairs: v,
            ..Default::default()
        }
    }
}

impl From<KvError> for CommandResponse {
    fn from(err: KvError) -> Self {
        let mut res = Self {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: err.to_string(),
            ..Default::default()
        };

        match err {
            KvError::NotFound(_, _) => res.status = StatusCode::NOT_FOUND.as_u16() as _,
            KvError::InvalidCommand(_) => res.status = StatusCode::BAD_REQUEST.as_u16() as _,
            _ => {}
        }

        res
    }
}

impl TryFrom<Value> for Vec<u8> {
    type Error = KvError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let val = if let Some(v) = &value.value {
            match v {
                crate::pb::value::Value::String(val) => Ok(val.as_bytes().to_vec()),
                _ => Err(KvError::ConvertError(value, "Vec[u8]")),
            }
        } else {
            Err(KvError::ConvertError(value, "Option::Some"))
        };

        val
    }
}
