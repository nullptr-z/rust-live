pub mod abi;

use abi::*;

impl LoginRequest {
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }

    // TODO：
    // jwt::encode(&header, &claims, &key)
    // use jwt to generate token`使用jwt生成token`
    // Claims用于存储用户信息，Header用于存储加密算法，Key用于加密
    pub fn into_token(&self) -> Token {
        Token::new(format!("{}:{}", &self.username, &self.password))
    }
}

impl Token {
    pub fn new(data: impl Into<String>) -> Self {
        Self { data: data.into() }
    }

    // TODO: use jwt for decode token
    pub fn into_usernmae(&self) -> String {
        self.data.clone()
    }

    pub fn is_valid(&self) -> bool {
        self.data.len() > 0
    }
}

impl NewChatMessage {
    pub fn new(room: impl Into<String>, conntent: impl Into<String>) -> Self {
        Self {
            room: room.into(),
            conntent: conntent.into(),
        }
    }

    pub fn into_chat_message(self, sender: impl Into<String>) -> ChatMessage {
        ChatMessage::new(sender, self.room, self.conntent)
    }
}

impl ChatMessage {
    pub fn new(
        sender: impl Into<String>,
        room: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        let timestamp = chrono::Utc::now().timestamp_millis();
        Self {
            sender: sender.into(),
            room: room.into(),
            content: content.into(),
            timestamp,
        }
    }
}

impl GetMessageRequest {
    pub fn new() -> Self {
        Self {}
    }
}
