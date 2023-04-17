mod msg;

use std::{
    collections::{HashMap, HashSet},
    sync::{mpsc::Receiver, Arc},
};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    Extension,
};
// DshMap: 相当于 RwLock<HashMap>，但是性能更好
// DashSet: 相当于 RwLock<HashSet>，但是性能更好
use dashmap::{DashMap, DashSet};
use futures::{SinkExt, StreamExt};
pub use msg::{Msg, MsgData};
use tokio::sync::broadcast;
use tracing::log::warn;

const CAPACITY: usize = 64;

#[derive(Debug)]
struct State {
    // for a given user, how many rooms ther'ye in`对于给定的用户，有多加入他的 rooms
    user_rooms: DashMap<String, DashSet<String>>,
    // for a given room, who's in it`对于给定的rooms，有哪些用户在里面
    room_users: DashMap<String, DashSet<String>>,
    // 这里的 tx 是一个广播通道，用于向所有的客户端发送消息
    tx: broadcast::Sender<Arc<Msg>>,
}

/// 为了在Extensions中使用，我们需要为`State`实现Clone
#[derive(Debug, Clone, Default)]
pub struct ChatState(Arc<State>);

impl Default for State {
    fn default() -> Self {
        // 这里不需要保留 rx,在后续使用时，通过调用 rx.subscribe() 返回一个 rx
        let (tx, _rx) = broadcast::channel(CAPACITY);
        Self {
            user_rooms: Default::default(),
            room_users: Default::default(),
            tx,
        }
    }
}

impl ChatState {
    pub fn new() -> Self {
        Self(Default::default())
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<ChatState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handl_socket(socket, state))
}

async fn handl_socket(socket: WebSocket, state: ChatState) {
    let mut rx = state.0.tx.subscribe();
    let (mut sende, mut receiver) = socket.split();

    // let state_cloned = state.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(msg) => {
                    handel_message(msg.as_str().try_into().unwrap(), state.0.clone()).await;
                }
                _ => (),
            }
        }
    });

    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let data = msg.as_ref().try_into().unwrap();
            if let Err(e) = sende.send(Message::Text(data)).await {
                warn!("error sending message`发送消息出现错误 : {}", e)
            }
        }
    });
}

async fn handel_message(msg: Msg, state: Arc<State>) {
    let msg = match msg.data {
        MsgData::Join => {
            // 1. 将 room 添加到 user_rooms 中
            state
                .user_rooms
                .entry(msg.usename.clone())
                .or_default()
                .insert(msg.room.clone());
            // 2. 将用户添加到 room_users 中
            state
                .room_users
                .entry(msg.room.clone())
                .or_default()
                .insert(msg.usename.clone());
            // 3. 将消息广播给 room 中的所有用户
            // let msg = Arc::new(msg);
            // state.0.tx.send(msg).unwrap();

            msg
        }
        MsgData::Leave => {
            // 如果用户在多个 romms 中，那么只需要将用户从 room_users 中移除
            // 如果最后一个 rooms 离开，那么需要将 rooms 从 user_rooms 中移除
            if let Some(v) = state.user_rooms.get_mut(&msg.usename) {
                v.remove(&msg.room);
                if v.is_empty() {
                    state.user_rooms.remove(&msg.usename);
                }
            };
            // 如果 room 中只有一个用户，那么需要将 room 从 room_users 中移除
            // 如果 room 中有多个用户，那么只需要将用户从 room_users 中移除
            if let Some(v) = state.room_users.get_mut(&msg.room) {
                v.remove(&msg.usename);
                if v.is_empty() {
                    state.room_users.remove(&msg.room);
                }
            };

            // let msg = Arc::new(msg);
            msg
        }
        _ => msg,
    };
    if let Err(e) = state.tx.send(Arc::new(msg)) {
        warn!("error sending message`发送消息出现错误 : {}", e)
    }
}
