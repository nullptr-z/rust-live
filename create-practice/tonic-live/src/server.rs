// use futures::prelude::Stream;
use crate::pb::abi::{
    chat_server::{Chat, ChatServer},
    ChatMessage, GetMessageRequest, LoginRequest, NewChatMessage, SendMessageResponse, Token,
};
use std::pin::Pin;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{
    codegen::futures_core::Stream, transport::Server, Extensions, Request, Response, Result, Status,
};
use tracing::{info, warn};

// 最大消息长度
const MAX_MESSAGE: usize = 1024;

pub type ChatResult<T> = Result<Response<T>, Status>;
pub type StatusResult<T> = Result<T, Status>;
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
        let sender = get_username(request.extensions())?;
        let info = request.into_inner();
        let message = info.into_chat_message(sender);

        // store it to the server storage`存储到服务器存储
        // publish message to everyone who interested in it`向所有对此感兴趣的人发布消息`
        self.tx.send(message).unwrap();

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
                    return;
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
    // inner: 客户端或服务器
    // with_interceptor是一个函数，用于为gRPC inner 添加拦截器。
    // 它接受一个 inner ，拦截器作为参数，并返回一个新的 inner
    // 该 inner 将使用提供的拦截器来处理所有传入和传出的请求和响应
    let svc = ChatServer::with_interceptor(ChatService::default(), check_auth);

    let addr = "0.0.0.0:8000".parse().unwrap();
    info!("listening on http://{addr}");

    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await
        .unwrap();
}

// gRPC拦截器，用于添加，删除 MetadataMap 选项
fn check_auth(mut req: Request<()>) -> Result<Request<()>, Status> {
    // 通过调用 metadata() 从 Header 中获取 token, 如果没有 token, 返回 unauthenticated 错误
    let token = match req.metadata().get("authorization") {
        Some(v) => {
            let token_str = v.to_str().map_err(|_| {
                Status::new(
                    tonic::Code::Unauthenticated,
                    "Invalid token format`令牌格式无效".to_string(),
                )
            })?;
            Token::new(token_str.strip_prefix("Bearer ").unwrap())
        }
        None => Token::default(),
    };

    req.extensions_mut().insert(token);
    Ok(req)
}

fn get_username(ext: &Extensions) -> StatusResult<String> {
    let token = ext
        .get::<Token>()
        .ok_or(Status::unauthenticated("No token"))?;

    if token.is_valid() {
        Ok(token.into_usernmae())
    } else {
        Err(Status::unauthenticated("Invalid token"))
    }
}
