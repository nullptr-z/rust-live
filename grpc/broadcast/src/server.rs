use std::pin::Pin;

use futures::Stream;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{transport::Server, Extensions, Request, Response, Status};
use tracing::{info, warn};

use crate::pb::broadcast::{
    broadcast_server::{self, BroadcastServer},
    BroadcastMessage, GetBroadcastRequest, LoginRequest, NewMessage, SendBroadcastResponse, Token,
};

const MAX_BROADCAST: usize = 1024;

pub struct BroadcastService {
    tx: broadcast::Sender<BroadcastMessage>,
}

type ServiceResult<T> = std::result::Result<Response<T>, tonic::Status>;

#[tonic::async_trait]
impl broadcast_server::Broadcast for BroadcastService {
    /// 登录
    async fn login(&self, request: tonic::Request<LoginRequest>) -> ServiceResult<Token> {
        let user_info = request.into_inner();
        info!("user_info: {:?}", user_info);
        let token = user_info.into_token();

        Ok(Response::new(token))
    }
    // Server streaming response type for the GetBroadcast method.
    // type GetBroadcastStream = tonic::Streaming<BroadcastMessage>;
    // futures_core::Stream<Item = Result<super::Broadcast, tonic::Status>> + Send + 'static;
    type GetBroadcastStream =
        Pin<Box<dyn Stream<Item = Result<BroadcastMessage, tonic::Status>> + Send>>;
    /// 订阅-接受
    async fn get_broadcast(
        &self,
        _request: tonic::Request<GetBroadcastRequest>,
    ) -> ServiceResult<Self::GetBroadcastStream> {
        // let info = request.into_inner();
        // info!("subscribe to message {info:?}");

        // 订阅广播消息
        let mut rx = self.tx.subscribe();
        let (sender, receiver) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            // 从广播通道中接收消息
            while let Ok(msg) = rx.recv().await {
                if let Err(_) = sender.send(Ok(msg)) {
                    warn!("Failed to send. Sender might be closed");
                    return;
                }
            }
        });

        let stream: UnboundedReceiverStream<Result<BroadcastMessage, _>> =
            UnboundedReceiverStream::new(receiver);
        Ok(Response::new(Box::pin(stream) as Self::GetBroadcastStream))
    }

    /// 发送-广播
    async fn send_broadcast(
        &self,
        request: tonic::Request<NewMessage>,
    ) -> ServiceResult<SendBroadcastResponse> {
        // 用户发起广播的请求
        let sender = get_username(&request.extensions())?;
        let info = request.into_inner();
        info!("send_broadcast:> {info:?} for sender: {sender:?}");
        // TODO: 从请求头 Token 中解析出用户信息
        // 从请求中提取用户要广播的内容
        let msg = info.into_broadcast_message(sender).unwrap();
        // publish message to everyone
        self.tx.send(msg).unwrap();

        Ok(Response::new(SendBroadcastResponse {}))
    }
}

impl Default for BroadcastService {
    fn default() -> Self {
        let (tx, _rx) = broadcast::channel(MAX_BROADCAST);
        Self { tx }
    }
}

pub async fn start() {
    let svc = BroadcastServer::with_interceptor(BroadcastService::default(), check_auth);
    let addr = "0.0.0.0:8080".parse().unwrap();
    info!("listening on http://{addr}");
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
            Token::new(data)
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
        let user = token.into_user();
        Ok(user.0)
    } else {
        Err(Status::unauthenticated("Invalid Token"))
    }
}
