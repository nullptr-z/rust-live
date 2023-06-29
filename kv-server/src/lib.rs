mod command;
mod error;
mod memory_db;
mod pb;

pub use command::*;
pub use error::KvError;
pub use memory_db::*;
pub use pb::{command_request::RequestData, CommandRequest, CommandResponse, Kvpair, Value};
use reqwest::StatusCode;

pub trait CommandServer {
    fn execute(self, store: &impl Storage) -> CommandResponse;

    fn verification_table<'a>(table: &'a str, res: &mut CommandResponse) -> Option<&'a str> {
        if table.is_empty() {
            res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _;
            res.message = format!("Not found Database Table: {}", table);
            return None;
        }
        Some(table)
    }
}

pub fn dispatch(cmd: CommandRequest, storage: &impl Storage) -> CommandResponse {
    if let Some(request_data) = cmd.request_data {
        return match request_data {
            RequestData::Hget(cmd) => cmd.execute(storage),
            RequestData::Hset(cmd) => cmd.execute(storage),
            RequestData::Hgetall(_) => todo!(),
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

pub trait Storage {
    /// 从一个 HashTable 里获取一个 key 的 value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 设置 HashTabl e里一个 key 的 value，返回旧的 value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    /// 查看 HashTable 中是否有 key
    fn contains(&self, table: &str, key: &str) -> Result<bool, Value>;
    /// 从 HashTable 中删除一个 key,返回被删除的这个 Value
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 遍历 HashTable，返回所有 kv pair（这个接口不好）
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    /// 遍历 HashTable，返回 kv pair 的 Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}
