mod message;

use anyhow::Result;
pub use message::*;
use tonic::metadata::AsciiMetadataValue;

impl Token {
    pub fn new(data: impl AsRef<str>) -> Self {
        Self {
            token: data.as_ref().into(),
        }
    }

    pub fn into_username(&self) -> String {
        self.token.clone()
    }

    pub fn is_valid(&self) -> bool {
        self.token.len() > 0
    }

    pub fn into_ascii_meatadata_bearer(&self) -> AsciiMetadataValue {
        AsciiMetadataValue::try_from(format!("Bearer {}", self.token).as_str()).unwrap()
    }
}

impl LoginRequest {
    pub fn new(username: impl Into<String>, password: impl AsRef<str>) -> Self {
        Self {
            username: username.into(),
            password: password.as_ref().into(),
        }
    }

    /// TODO: use jwt token instead
    pub fn into_token(&self) -> Token {
        Token::new(&self.username)
    }
}

impl NewChatMessage {
    pub fn new(room: impl Into<String>, content: impl AsRef<str>) -> Self {
        Self {
            room: room.into(),
            content: content.as_ref().into(),
        }
    }

    pub fn into_chat_message(self, sender: impl Into<String>) -> ChatMessage {
        ChatMessage::new(self.room, self.content, sender)
    }
}

impl ChatMessage {
    pub fn new(
        room: impl Into<String>,
        content: impl AsRef<str>,
        sender: impl Into<String>,
    ) -> Self {
        let timestamp = chrono::Utc::now().timestamp();

        Self {
            sender: sender.into(),
            room: room.into(),
            content: content.as_ref().into(),
            timestamp,
        }
    }
}

impl GetMessageRequest {
    pub fn new() -> Self {
        Self {}
    }
}
