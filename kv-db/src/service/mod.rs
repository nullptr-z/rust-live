mod command_service;
pub mod notify;
pub mod service_builder;
pub mod topic;

use crate::{
    error::KvError,
    memory::MemTable,
    pb::abi::{command_request::RequestData, CommandRequest, CommandResponse},
    Storage,
};
use std::{ops::Deref, sync::Arc};
use tracing::info;

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

/// 可以跨线程，可以调用 execute 来执行某个 CommandRequest 命令，返回 CommandResponse。
pub struct Service<Store = MemTable> {
    inner: Arc<ServiceBuilder<Store>>,
}

impl<Store: Storage> Service<Store> {
    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        info!("God request: {:?}", &cmd);
        self.on_received.notify(&cmd);
        let mut resp = dispatch(cmd, &self.store);
        info!("Executed response: {:?}", resp);
        self.on_executed.notify(&resp);
        self.on_before_send.notify(&mut resp);

        if !self.on_before_send.is_empty() {
            info!("Modified response: {:?}", resp);
        }

        resp
    }
}

impl<Store> Clone for Service<Store> {
    fn clone(&self) -> Self {
        Service {
            inner: self.inner.clone(),
        }
    }
}

impl<Store> Deref for Service<Store> {
    type Target = Arc<ServiceBuilder<Store>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(RequestData::Hget(cmd)) => cmd.execute(store),
        Some(RequestData::Hgetall(cmd)) => cmd.execute(store),
        Some(RequestData::Hset(cmd)) => cmd.execute(store),
        None => KvError::InvalidCommand("Request has no data".into()).into(),
        _ => KvError::Internal("Not implemented yet".into()).into(),
    }
}

#[cfg(test)]
mod service_tests {
    use super::*;
    use crate::{
        memory::MemTable,
        pb::abi::{command_request::RequestData, CommandRequest, CommandResponse, Kvpair, Value},
        Storage,
    };

    #[test]
    fn hset_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("t1", "hello", "world");
        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &[Value::default()], &[]);

        let res = dispatch(cmd, &store);
        assert_res_ok(res, &["world".into()], &[]);
    }

    #[test]
    fn hget_should_work() {
        let store = MemTable::new();
        let cmd = CommandRequest::new_hset("score", "u1", 10);
        dispatch(cmd, &store);
        let cmd = CommandRequest::new_hget("score", "u1");
        let res = dispatch(cmd, &store);
        assert_res_ok(res, &[10.into()], &[]);
    }

    #[test]
    fn hgetall_should_work() {
        let store = MemTable::new();
        let cmds = vec![
            CommandRequest::new_hset("score", "u1", 10),
            CommandRequest::new_hset("score", "u2", 8),
            CommandRequest::new_hset("score", "u3", 11),
            CommandRequest::new_hset("score", "u1", 6),
        ];
        for cmd in cmds {
            dispatch(cmd, &store);
        }
    }

    // 测试成功返回的结果
    fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
        res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(res.status, 200);
        assert_eq!(res.message, "");
        assert_eq!(res.values, values);
        assert_eq!(res.pairs, pairs);
    }

    // 测试失败返回的结果
    fn assert_res_error(res: CommandResponse, code: u32, msg: &str) {
        assert_eq!(res.status, code);
        assert!(res.message.contains(msg));
        assert_eq!(res.values, &[]);
        assert_eq!(res.pairs, &[]);
    }

    fn dispatch(mut cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
        match cmd.request_data.unwrap() {
            RequestData::Hget(v) => v.execute(store),
            RequestData::Hgetall(v) => v.execute(store),
            RequestData::Hset(v) => v.execute(store),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod service_tests_2 {
    use std::thread;

    use crate::{
        assert_res_ok,
        pb::abi::{CommandRequest, Value},
        Service,
    };

    use super::service_builder::ServiceBuilder;

    #[test]
    fn service_should_works() {
        // 我们需要一个 service 结构至少包含 Storage
        let service: Service = ServiceBuilder::default().finish();

        // service 可以运行在多线程环境下，它的 clone 应该是轻量级的
        let cloned = service.clone();

        // 创建一个线程，在 table t1 中写入 k1, v1
        let handle = thread::spawn(move || {
            let res = cloned.execute(CommandRequest::new_hset("t1", "k1", "v1"));
            assert_res_ok(res, &[Value::default()], &[]);
        });
        handle.join().unwrap();

        // 在当前线程下读取 table t1 的 k1，应该返回 v1
        let res = service.execute(CommandRequest::new_hget("t1", "k1"));
        assert_res_ok(res, &["v1".into()], &[]);
    }
}

#[cfg(test)]
use crate::pb::abi::{Kvpair, Value};

use self::{
    notify::{Notify, NotifyMut},
    service_builder::ServiceBuilder,
};
// 测试成功返回的结果
#[cfg(test)]
pub fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[Kvpair]) {
    res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(res.status, 200);
    assert_eq!(res.message, "");
    assert_eq!(res.values, values);
    assert_eq!(res.pairs, pairs);
}

// 测试失败返回的结果
#[cfg(test)]
pub fn assert_res_error(res: CommandResponse, code: u32, msg: &str) {
    assert_eq!(res.status, code);
    assert!(res.message.contains(msg));
    assert_eq!(res.values, &[]);
    assert_eq!(res.pairs, &[]);
}
