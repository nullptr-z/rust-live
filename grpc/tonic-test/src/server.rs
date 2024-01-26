use crate::abi::{
    chat_server::{Chat, ChatServer},
    *,
};
use futures::prelude::*;
use std::{pin::Pin, result::Result};
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{transport::Server, Extensions, Request, Response, Status};
use tracing::{info, warn};

const MAX_MESSAGE: usize = 1024;

pub struct ChatService {
    tx: broadcast::Sender<ChatMessage>,
}

pub type ChatResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl Chat for ChatService {
    // 获取登录用户信息，返回用户token
    async fn login(&self, request: Request<LoginRequest>) -> ChatResult<Token> {
        let info = request.into_inner();
        let token = info.into_token();
        Ok(Response::new(token))
    }

    async fn send_message(
        &self,
        request: Request<NewChatMessage>,
    ) -> ChatResult<SendMessageResponse> {
        // 从请求中获取用户信息，这里的metadata类似http的header
        let sender = get_username(request.extensions())?;
        let info = request.into_inner();
        info!("send_message:> {info:?}");
        let msg = info.into_chat_message(sender);
        // publish message to everyone
        self.tx.send(msg).unwrap();
        Ok(Response::new(SendMessageResponse {}))
    }

    type GetMessageStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, Status>> + Send>>;
    async fn get_message(
        &self,
        request: Request<GetMessageRequest>,
    ) -> ChatResult<Self::GetMessageStream> {
        let info = request.into_inner();
        info!("subscript to message {info:?}");
        let mut rx = self.tx.subscribe();
        let (sender, receive) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if let Err(_) = sender.send(Ok(msg)) {
                    warn!("Failed to send. Sender might be closed.");
                    return;
                }
            }
        });

        let stream = UnboundedReceiverStream::new(receive);

        Ok(Response::new(Box::pin(stream)))
    }
}

pub async fn start() {
    let svc = ChatServer::with_interceptor(ChatService::default(), check_auth);
    let addr = "0.0.0.0:8000".parse().unwrap();
    info!("listening on http://{:?}", addr);
    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .unwrap();
}

fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    let token = match req.metadata().get("authorization") {
        Some(v) => {
            let data = v
                .to_str()
                .map_err(|_| Status::new(tonic::Code::Unauthenticated, "Invalid token format"))?;
            Token::new(data.strip_prefix("Bearer ").unwrap())
        }
        None => Token::default(),
    };
    req.extensions_mut().insert(token);
    Ok(req)
}

fn get_username(ext: &Extensions) -> Result<String, Status> {
    let token = ext
        .get::<Token>()
        .ok_or(Status::unauthenticated("no token"))?;
    if token.is_valid() {
        Ok(token.into_username())
    } else {
        Err(Status::unauthenticated("Invalid Token"))
    }
}

pub trait MetadataExt {
    fn flip(self) -> String;
}

impl MetadataExt for Option<&tonic::metadata::MetadataValue<tonic::metadata::Ascii>> {
    fn flip(self) -> String {
        self.unwrap().to_str().unwrap().to_string()
    }
}

impl Default for ChatService {
    fn default() -> Self {
        let (tx, _rx) = broadcast::channel(MAX_MESSAGE);

        Self { tx }
    }
}
