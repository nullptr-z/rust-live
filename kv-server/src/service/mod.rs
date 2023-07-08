mod command_server;

use std::sync::Arc;

pub use command_server::*;
use tracing::debug;

use crate::{CommandRequest, CommandResponse, MemoryDB, RequestData, Storage};

pub trait CommandServer {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

// 这是 Rust 的一个惯例，把需要在多线程下 clone 的主体和其内部结构分开，这样代码逻辑更加清晰。
pub struct ServiceInner<Store> {
    pub store: Store,
    // 消息函数
    /// on_received：当服务器收到 CommandRequest 时触发；
    pub on_recevied: Vec<fn(&CommandRequest)>,
    /// on_executed：当服务器处理完 CommandRequest 得到 CommandResponse 时触发；
    pub on_executed: Vec<fn(&CommandResponse)>,
    /// on_before_send：在服务器发送 CommandResponse 之前触发。注意这个接口提供的是 &mut CommandResponse，这样事件的处理者可以根据需要，在发送前，修改 CommandResponse。
    pub on_before_send: Vec<fn(&mut CommandResponse)>,
    /// on_after_send：在服务器发送完 CommandResponse 后触发。
    pub on_after_send: Vec<fn()>,
}

impl<Store> ServiceInner<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            store,
            on_recevied: vec![],
            on_executed: vec![],
            on_before_send: vec![],
            on_after_send: vec![],
        }
    }

    pub fn fn_recevied(mut self, middlware: fn(&CommandRequest)) -> Self {
        self.on_recevied.push(middlware);
        self
    }

    pub fn fn_executed(mut self, middlware: fn(&CommandResponse)) -> Self {
        self.on_executed.push(middlware);
        self
    }

    pub fn fn_before_send(mut self, middlware: fn(&mut CommandResponse)) -> Self {
        self.on_before_send.push(middlware);
        self
    }

    pub fn fn_after_send(mut self, middlware: fn()) -> Self {
        self.on_after_send.push(middlware);
        self
    }
}

pub struct Service<Store = MemoryDB> {
    inner: Arc<ServiceInner<Store>>,
}

pub trait Notify<Arg> {
    fn notify(&self, arg: &Arg);
}

pub trait NotifyMut<Arg> {
    fn notify(&self, arg: &mut Arg);
}

impl<Arg> Notify<Arg> for Vec<fn(&Arg)> {
    #[inline]
    fn notify(&self, arg: &Arg) {
        for f in self {
            f(arg)
        }
    }
}
impl<Arg> NotifyMut<Arg> for Vec<fn(&mut Arg)> {
    #[inline]
    fn notify(&self, arg: &mut Arg) {
        for f in self {
            f(arg)
        }
    }
}

impl<Store: Storage> Service<Store> {
    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        self.inner.on_recevied.notify(&cmd);
        let mut resp = dispatch(cmd, &self.inner.clone().store);
        self.inner.on_executed.notify(&resp);
        self.inner.on_before_send.notify(&mut resp);
        if !self.inner.on_before_send.is_empty() {
            debug!("Modified`修改后的 response: {:?}", resp);
        }

        resp
    }
}

impl<Store> From<ServiceInner<Store>> for Service<Store> {
    fn from(inner: ServiceInner<Store>) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

// 支持 Arc clone
impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

pub fn dispatch(cmd: CommandRequest, storage: &impl Storage) -> CommandResponse {
    if let Some(request_data) = cmd.request_data {
        return match request_data {
            RequestData::Hget(cmd) => cmd.execute(storage),
            RequestData::Hset(cmd) => cmd.execute(storage),
            RequestData::Hgetall(cmd) => cmd.execute(storage),
            RequestData::Hmget(_) => todo!(),
            RequestData::Hmset(_) => todo!(),
            RequestData::Hdel(_) => todo!(),
            RequestData::Hmdel(_) => todo!(),
            RequestData::Hexist(_) => todo!(),
            RequestData::Hmexist(_) => todo!(),
        };
    }
    CommandResponse::default()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_service() {
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        let function_pointer: fn(i32, i32) -> i32 = add;
    }
}
