// use futures::prelude::Stream;
use crate::pb::abi::{
    chat_server::{Chat, ChatServer},
    ChatMessage, GetMessageRequest, LoginRequest, NewChatMessage, SendMessageResponse, Token,
};
use std::pin::Pin;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{
    codegen::futures_core::Stream, service::interceptor, transport::Server, Request, Response,
    Result, Status,
};
use tracing::{info, log::warn};

// 最大消息长度
const MAX_MESSAGE: usize = 1024;

pub type ChatResult<T> = Result<Response<T>, Status>;
pub struct ChatService {
    // 广播
    tx: broadcast::Sender<ChatMessage>,
}

/// 实现
#[tonic::async_trait]
impl Chat for ChatService {
    async fn login(&self, request: Request<LoginRequest>) -> ChatResult<Token> {
        let info = request.into_inner(); // 获取内部对象的所有权
        info!("login:{info:?}");
        let token = info.into_token();

        Ok(Response::new(token))
    }

    async fn send_message(
        &self,
        request: Request<NewChatMessage>,
    ) -> ChatResult<SendMessageResponse> {
        // how to get sender, from request or from token?`如何从请求或令牌中获取发送者？
        let sender = "todo!()";
        let info = request.into_inner();
        let message = info.into_chat_message(sender);

        // store it to the server storage`存储到服务器存储
        // publish message to everyone who interested in it`向所有对此感兴趣的人发布消息`
        Ok(Response::new(SendMessageResponse {}))
    }

    type GetMessageStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, Status>> + Send>>;

    async fn get_message(
        &self,
        request: Request<GetMessageRequest>,
    ) -> ChatResult<Self::GetMessageStream> {
        let info = request.into_inner();
        info!("subscribe to message {info:?}");
        let mut rx = self.tx.subscribe(); // 订阅
        let (sender, receiver) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            // TODO: filter out message I'm not interested in`过滤掉我不感兴趣的消息
            while let Ok(msg) = rx.recv().await {
                if let Err(_) = sender.send(Ok(msg)) {
                    warn!("Failed to send. Sender might be closed.");
                }
            }
        });

        let stream = UnboundedReceiverStream::new(receiver);
        Ok(Response::new(Box::pin(stream)))
    }
}

impl Default for ChatService {
    fn default() -> Self {
        let (tx, _rx) = broadcast::channel(MAX_MESSAGE);

        Self { tx }
    }
}

pub async fn start() {
    let svc = ChatServer::with_interceptor(ChatService::default(), check_auth);

    let addr = "0.0.0.0".parse().unwrap();
    info!("listening on http://{addr}");

    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .unwrap();
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    // 通过调用metadata从Header中获取token, 如果没有token, 返回unauthenticated错误
    let token = match req.metadata().get("authorization") {
        Some(v) => {
            let token_str = v.to_str().map_err(|_| {
                Status::new(
                    tonic::Code::Unauthenticated,
                    "Invalid token format`令牌格式无效".to_string(),
                )
            })?;
            let token = Token::new(token_str.strip_prefix("Bearer ").unwrap());
            let username = token.into_usernmae();
        }
        None => return Err(Status::unauthenticated("missing token")),
    };
    todo!()
}
