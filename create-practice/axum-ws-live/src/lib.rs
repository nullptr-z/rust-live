mod msg;

use std::{fmt::Debug, sync::Arc};

use axum::{
    extract::{ws::Message, WebSocketUpgrade},
    response::IntoResponse,
    Extension,
};
// DshMap: 相当于 RwLock<HashMap>，但是性能更好
// DashSet: 相当于 RwLock<HashSet>，但是性能更好
use dashmap::{DashMap, DashSet};
use futures::{Sink, SinkExt, Stream, StreamExt};
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

    // 返回 username 用户所有 room, 如果没有返回空的 Vec
    pub fn get_user_rooms(&self, username: &str) -> Vec<String> {
        self.0
            .user_rooms
            .get(username)
            .map(|room| room.clone().into_iter().collect())
            // 如果这个用户没有room，返回一个空的 Vec
            .unwrap_or_default()
    }

    // 返回 room 所有的用户user, 如果没有返回空的 Vec
    pub fn get_room_users(&self, room: &str) -> Vec<String> {
        self.0
            .room_users
            .get(room)
            .map(|user| user.clone().into_iter().collect())
            .unwrap_or_default()
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    // claims: Cliams,
    Extension(state): Extension<ChatState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handl_socket(socket, state))
}

async fn handl_socket<S>(socket: S, state: ChatState)
where
    // 这里的 S 是一个异步流，用于接收客户端的消息
    // 'static 表示这个流的生命周期是整个程序,是一个 owned 的类型,这是必要的，因为我们需要将它传递给 tokio::spawn
    // 如果 S是一个 Reference，那么它的生命周期就是这个函数，而不是整个程序
    S: Stream<Item = Result<Message, axum::Error>> + Sink<Message> + 'static + Send,
{
    // 这里的 rx 是一个广播通道，用于向所有的客户端发送消息
    let mut rx = state.0.tx.subscribe();
    // 将socket分成发送者和接收者
    let (mut sende, mut receiver) = socket.split();

    let state1 = state.clone();
    // 不停地从客户端接收` receiver.next()`消息`Message`，并处理消息`handel_message`
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(msg) => {
                    handel_message(msg.as_str().try_into().unwrap(), state1.0.clone()).await;
                }
                _ => (),
            }
        }
    });

    // 从广播通道接收消息`rx.recv`，并发送给客户端`sende.send`
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let data = msg.as_ref().try_into().unwrap();
            if let Err(e) = sende.send(Message::Text(data)).await {
                warn!("error sending message`发送消息出现错误");
                break;
            }
        }
    });

    // if any of the tasks fail, we need to shut down the other one`如果任意任务(receiver or send)失败，则另一个任务也需要关闭。
    tokio::select! {
        _v1 =&mut recv_task => {
            // 如果接收任务失败，那么发送任务也需要关闭
            send_task.abort();
        }
        _v2 =&mut send_task => {
            // 如果发送任务失败，那么接收任务也需要关闭
            recv_task.abort();
        }
    }

    // this user has left. Should send a leave message to all other users in the room`这个用户已经离开了。应该向room中的所有其他用户发送离开消息
    // usually we can get username from auth header, here we just use "fake_user"`通常我们可以从 auth header 中获取 username，这里只是使用 "fake_user"模拟
    let username = "fake_user";
    warn!("connection for {username } closed`连接关闭");
    let rooms = state.get_user_rooms(username);
    for room in rooms {
        if let Err(e) = state.0.tx.send(Arc::new(Msg::leave(&room, username))) {
            warn!("failed to send leave message`发送失败信息 : {e}");
        }
    }
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
