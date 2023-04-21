// 全局的客户端链接,多线程访问
// 当前链接的用户
// 用户当前访问的 room[]
// room 对应的 message

use crate::pb::abi::{
    chat_client::ChatClient, ChatMessage, GetMessageRequest, LoginRequest, NewChatMessage, Token,
};
use anyhow::Result;
use arc_swap::ArcSwap;
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::{ops::Deref, sync::Arc};
use tonic::{codegen::InterceptedService, service::Interceptor, transport::Channel};

lazy_static! {
    // ArcSwap是一种无锁同步原语,它提供了一种在无锁的情况下原子地替换共享变量值的方式，而不需要全局锁或其他形式的同步
    static ref TOKEN: ArcSwap<Token> = ArcSwap::from(Arc::new(Token {
        data: "".to_string()
    }));
}

#[derive(Debug, Default, Clone)]
struct Rooms(Arc<DashMap<String, Vec<ChatMessage>>>);

pub struct Client {
    username: String,
    conn: ChatClient<InterceptedService<Channel, AuthInterceptor>>,
    // 收到的所有 Message 存储到相应的 room 中
    rooms: Rooms,
}

impl Deref for Rooms {
    type Target = Arc<DashMap<String, Vec<ChatMessage>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Rooms {
    // 向 Rooms 中添加 message
    fn insert_message(&self, msg: ChatMessage) {
        let room = msg.room.clone();
        // 通过 room 名称取出同名 room[]，如果没有则创建一个空的
        let mut room_message = self.entry(room).or_insert_with(Vec::new);
        room_message.push(msg);
    }
}

impl Client {
    pub async fn connect_server(username: String) -> Self {
        // 创建一个链接到服务端的服务
        let channel = Channel::from_static("http://[127.0.0.1]:8000")
            .connect()
            .await
            .unwrap();

        // 将链接封装成 gRPC 的链接
        // 通过 with_interceptor 创建可以为 gRPC 的链接添加拦截器，如果不需要可以使用 ChatClient::New
        let conn = ChatClient::with_interceptor(channel, AuthInterceptor);

        Self {
            username,
            conn,
            rooms: Default::default(),
        }
    }

    pub async fn login(&mut self) -> Result<()> {
        let login = LoginRequest::new(&self.username, "password");
        let token = self.conn.login(login).await?.into_inner();
        // 在所有多线程中共享 token
        TOKEN.store(Arc::new(token));

        Ok(())
    }

    pub async fn send_message(
        &mut self,
        room: impl Into<String>,
        content: impl Into<String>,
    ) -> Result<()> {
        let msg = NewChatMessage::new(room, content);
        self.conn.send_message(msg).await?;

        Ok(())
    }

    pub async fn get_messages(&mut self) -> Result<()> {
        let req = GetMessageRequest::new();
        let mut stream = self.conn.get_message(req).await?.into_inner();

        let rooms = self.rooms.clone();
        tokio::spawn(async move {
            while let Some(msg) = stream.message().await? {
                rooms.insert_message(msg);
            }
            Ok::<_, tonic::Status>(())
        });

        Ok(())
    }
}

struct AuthInterceptor;

impl Interceptor for AuthInterceptor {
    fn call(&mut self, _request: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
        todo!()
    }
}
