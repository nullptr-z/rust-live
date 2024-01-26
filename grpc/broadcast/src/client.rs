use crate::pb::broadcast::{
    broadcast_client::BroadcastClient, BroadcastMessage, GetBroadcastRequest, LoginRequest,
    NewMessage, Token,
};
use anyhow::Result;
use arc_swap::ArcSwap;
use dashmap::DashMap;
use std::{borrow::Cow, ops::Deref, str::FromStr, sync::Arc};
use tonic::{
    codegen::InterceptedService, metadata::AsciiMetadataValue, service::Interceptor,
    transport::Channel, Status,
};
use tracing::info;

// 用于存储token，这里用了一个全局变量，实际项目中，可以用redis或者其他的存储
lazy_static::lazy_static! {
    static ref TOKEN:ArcSwap<Token> =ArcSwap::from(Arc::new(Token::default()));
}

#[derive(Debug, Default, Clone)]
struct Rooms(Arc<DashMap<String, Vec<BroadcastMessage>>>);

pub struct Client {
    username: String,
    conn: BroadcastClient<InterceptedService<Channel, AuthInterceptor>>,
    rooms: Rooms,
}

impl Deref for Rooms {
    type Target = DashMap<String, Vec<BroadcastMessage>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Rooms {
    pub fn insert_message(&self, msg: BroadcastMessage) {
        self.entry(msg.room.clone()).or_default().push(msg);
    }
}

impl Client {
    pub async fn new(username: impl Into<String>) -> Self {
        // 链接服务器
        let channel = Channel::from_static("http://127.0.0.1:8080")
            .connect()
            .await
            .unwrap();
        // 注册中间件，挂载token
        let conn = BroadcastClient::with_interceptor(channel, AuthInterceptor);

        Self {
            username: username.into(),
            conn,
            rooms: Rooms::default(),
        }
    }

    pub async fn login(&mut self) -> Result<()> {
        let userid = Cow::Borrowed("user—id xx");
        let pwd = std::borrow::Cow::Borrowed("password");
        let login = LoginRequest::new(userid, &self.username, pwd.to_owned());
        let token = self.conn.login(login).await?.into_inner();
        TOKEN.store(Arc::new(token));

        Ok(())
    }

    pub async fn send_message(
        &mut self,
        room: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<()> {
        let msg = NewMessage::new(content, room);
        self.conn.send_broadcast(msg).await?;

        Ok(())
    }

    pub async fn get_message(&mut self) -> Result<()> {
        let req = GetBroadcastRequest::new();
        // 从服务器获取广播消息，流
        let mut stream = self.conn.get_broadcast(req).await?.into_inner();

        let rooms = self.rooms.clone();
        // 等待订阅的消息到来
        tokio::spawn(async move {
            while let Some(msg) = stream.message().await? {
                info!("subscripted message> {:?}", msg);
                // 把到来的消息，存储到对应的room
                rooms.insert_message(msg);
            }
            Ok::<_, Status>(())
        });

        Ok(())
    }
}

struct AuthInterceptor;

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut req: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        // 获取缓存的token
        let token = TOKEN.load();

        if token.is_valid() {
            let bearer_token_ascii =
                AsciiMetadataValue::try_from(format!("Bearer {}", token.token)).unwrap();

            req.metadata_mut()
                .insert("authorization", bearer_token_ascii);
        }

        Ok(req)
    }
}
