pub mod abi;

use abi::{command_request::RequestData, *};

impl CommandRequest {
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(Hset::new(table, key, value).into()),
        }
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

impl Hset {
    pub fn new(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            table: table.into(),
            pair: Some(Kvpair::new(key, value)),
        }
    }
}

impl From<Hset> for RequestData {
    fn from(value: Hset) -> Self {
        RequestData::Hset(value)
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
