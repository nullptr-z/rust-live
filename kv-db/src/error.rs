use crate::pb::abi::Value;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum KvError {
    #[error("Not found for table: {0}, key: {1}")]
    NotFound(String, String),

    #[error("Cannot parse command: `{0}`")]
    InvalidCommand(String),
    #[error("Cannot convert value {:0} to {1}")]
    ConvertError(Value, &'static str),
    #[error("Cannot process command {0} with table: {1}, key: {2}. Error: {}")]
    StorageError(&'static str, String, String, String),

    #[error("Failed to encode protobuf message")]
    EncodeError(#[from] prost::EncodeError),
    #[error("Failed to decode protobuf message")]
    DecodeError(#[from] prost::DecodeError),
    #[error("std::io error: {0}")]
    IOError(String),
    // IOError(#[from] std::io::Error),
    #[error("Internal error: {0}")]
    Internal(String),
}

pub(crate) trait IOError<T> {
    fn to_error(self) -> Result<T, KvError>;
}

impl<T> IOError<T> for Result<T, std::io::Error> {
    fn to_error(self) -> Result<T, KvError> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(KvError::IOError(format!("{:?}", e))),
        }
    }
}
