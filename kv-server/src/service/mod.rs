mod command_server;

use std::sync::Arc;

pub use command_server::*;

use crate::{CommandRequest, CommandResponse, MemoryDB, RequestData, Storage};

pub trait CommandServer {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

// 这是 Rust 的一个惯例，把需要在多线程下 clone 的主体和其内部结构分开，这样代码逻辑更加清晰。
struct ServiceInner<Store> {
    store: Store,
}

pub struct Service<Store = MemoryDB> {
    inner: Arc<ServiceInner<Store>>,
}

impl<Store: Storage> Service<Store> {
    pub fn new(store: Store) -> Self {
        Self {
            inner: Arc::new(ServiceInner { store }),
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        let resp = dispatch(cmd, &self.inner.clone().store);
        resp
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
    fn test_service() {}
}
