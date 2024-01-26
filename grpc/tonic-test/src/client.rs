use crate::abi::{chat_client::ChatClient, *};
use anyhow::Result;
use arc_swap::ArcSwap;
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::{ops::Deref, sync::Arc};
use tonic::{codegen::InterceptedService, service::Interceptor, transport::Channel, Status};
use tracing::info;

lazy_static! {
    static ref TOKEN: ArcSwap<Token> = ArcSwap::from(Arc::new(Token {
        token: "".to_string()
    }));
}

// map存放着所有room以及，每个room里的消息(vec 表)
#[derive(Default, Clone)]
struct Rooms(Arc<DashMap<String, Vec<ChatMessage>>>);

impl Rooms {
    fn insert_message(&self, msg: ChatMessage) {
        let room = msg.room.clone();
        let mut room_message = self.entry(room).or_insert_with(Vec::new);
        room_message.push(msg);
    }
}

impl Deref for Rooms {
    type Target = DashMap<String, Vec<ChatMessage>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Client {
    username: String,
    conn: ChatClient<InterceptedService<Channel, AuthInterceptor>>,
    rooms: Rooms,
}

impl Client {
    pub async fn new(username: impl Into<String>) -> Self {
        let channel = Channel::from_static("http://127.0.0.1:8000")
            .connect()
            .await
            .unwrap();
        let conn = ChatClient::with_interceptor(channel, AuthInterceptor);

        Self {
            username: username.into(),
            conn,
            rooms: Default::default(),
        }
    }

    pub async fn login(&mut self) -> Result<()> {
        let login = LoginRequest::new(&self.username, "password");
        let token = self.conn.login(login).await?.into_inner();
        TOKEN.store(Arc::new(token));
        Ok(())
    }

    pub async fn send_message(
        &mut self,
        room: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<()> {
        let msg = NewChatMessage::new(room, content.into());
        self.conn.send_message(msg).await?;

        Ok(())
    }

    pub async fn get_message(&mut self) -> Result<()> {
        let req = GetMessageRequest::new();
        let mut stream = self.conn.get_message(req).await?.into_inner();

        let rooms = self.rooms.clone();
        tokio::spawn(async move {
            while let Some(msg) = stream.message().await? {
                info!("got message:> {:?}", msg);
                rooms.insert_message(msg)
            }
            Ok::<_, Status>(())
        });

        Ok(())
    }
}

struct AuthInterceptor;

impl Interceptor for AuthInterceptor {
    fn call(&mut self, mut request: tonic::Request<()>) -> Result<tonic::Request<()>, Status> {
        let token = TOKEN.load();
        if token.is_valid() {
            let bearer_token = Token::new(token.token.to_owned()).into_ascii_meatadata_bearer();
            request.metadata_mut().insert("authorization", bearer_token);
        }
        Ok(request)
    }
}
