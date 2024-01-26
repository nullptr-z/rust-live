use crate::jwt::{parse, signing, verification};
use anyhow::Result;
use std::borrow::Cow;

use self::broadcast::{BroadcastMessage, GetBroadcastRequest, LoginRequest, NewMessage, Token};

pub mod broadcast;

impl Token {
    pub fn new(info: impl AsRef<str>) -> Self {
        let token = signing(info);
        Self { token }
    }

    pub fn into_user(&self) -> (String, String) {
        let info = parse(&self.token);

        info
    }

    pub fn is_valid(&self) -> bool {
        self.token.len() > 0
    }
}

impl LoginRequest {
    pub fn new(userid: Cow<str>, username: impl AsRef<str>, pwd: Cow<str>) -> Self {
        Self {
            userid: userid.into_owned(),
            username: username.as_ref().to_string(),
            password: pwd.into_owned(),
        }
    }

    pub fn into_token(&self) -> Token {
        Token::new(format!("{}+{}", self.username, self.password))
    }
}

impl NewMessage {
    pub fn new(content: impl Into<String>, room: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            room: room.into(),
        }
    }

    pub fn into_broadcast_message(self, sender: impl Into<String>) -> Result<BroadcastMessage> {
        Ok(BroadcastMessage::new(self.content, self.room, sender))
    }
}

impl BroadcastMessage {
    pub fn new(
        content: impl Into<String>,
        room: impl Into<String>,
        sender: impl Into<String>,
    ) -> Self {
        let timestamp = chrono::Utc::now().timestamp();
        Self {
            content: content.into(),
            room: room.into(),
            sender: sender.into(),
            timestamp,
        }
    }
}

impl GetBroadcastRequest {
    pub fn new() -> Self {
        Self {}
    }
}
