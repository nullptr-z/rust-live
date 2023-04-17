use std::time::SystemTime;

use serde::{Deserialize, Serialize};

/// 用于在客户端和服务器之间传递消息的结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct Msg {
    pub room: String,
    pub usename: String,
    pub timestamp: u64,
    pub data: MsgData,
}

/// 用于在客户端和服务器之间传递消息的数据类型
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")] // rename_all = "snake_case"作用：将枚举变量转换为小写
pub enum MsgData {
    Join,
    Leave,
    Message(String),
}

impl Msg {
    pub fn new(room: impl Into<String>, usename: impl Into<String>, data: MsgData) -> Self {
        Msg {
            room: room.into(),
            usename: usename.into(),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data,
        }
    }

    pub fn join(room: &str, username: &str) -> Self {
        Msg::new(room, username, MsgData::Join)
    }

    pub fn leave(room: &str, username: &str) -> Self {
        Msg::new(room, username, MsgData::Leave)
    }

    pub fn message(room: &str, username: &str, message: &str) -> Self {
        Msg::new(room, username, MsgData::Message(message.into()))
    }
}

impl TryFrom<&str> for Msg {
    type Error = serde_json::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(s)
    }
}

impl TryFrom<&Msg> for String {
    type Error = serde_json::Error;
    fn try_from(msg: &Msg) -> Result<Self, Self::Error> {
        serde_json::to_string(&msg)
    }
}
