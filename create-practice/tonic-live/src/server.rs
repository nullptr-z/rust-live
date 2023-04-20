// use futures::prelude::Stream;
use std::pin::Pin;
use tonic::{codegen::futures_core::Stream, Request, Response, Result, Status};
use tracing::info;

use crate::pb::abi::{
    chat_server::Chat, ChatMessage, GetMessageRequest, LoginRequest, NewChatMessage,
    SendMessageResponse, Token,
};

pub struct ChatService;

pub type ChatResult<T> = Result<Response<T>, Status>;

/// 实现
#[tonic::async_trait]
impl Chat for ChatService {
    async fn login(&self, request: Request<LoginRequest>) -> ChatResult<Token> {
        let info = request.into_inner(); // 获取内部对象的所有权
        info!("login:{info:?}");
        let token = info.into_token();

        Ok(Response::new(token));
    }

    async fn send_message(
        &self,
        request: Request<NewChatMessage>,
    ) -> ChatResult<SendMessageResponse> {
        // how to get sender, from request or from token?
        let sender = "todo!()";
        let info = request.into_inner();
        let message = info.into_chat_message(sender);

        // store it to the server storage
        // publish message to everyone who interrested in it`
        Ok(Response::new(SendMessageResponse {}))
    }

    type GetMessageStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, Status>> + Send>>;

    async fn get_message(
        &self,
        request: Request<GetMessageRequest>,
    ) -> ChatResult<Self::GetMessageStream> {
        let info = request.into_inner();
        println!("【 info 】==> {:?}", info);

        todo!()
    }
}
