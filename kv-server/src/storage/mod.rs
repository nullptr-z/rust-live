mod memory_db;

pub use memory_db::*;

use crate::{KvError, Kvpair, Value};

pub trait Storage {
    /// 从一个 HashTable 里获取一个 key 的 value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 设置/更新 HashTabl e里一个 key 的 value，返回旧的 value \
    /// 首次设置返回的是 `None`
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    /// 查看 HashTable 中是否有 key
    fn contains(&self, table: &str, key: &str) -> Result<bool, Value>;
    /// 从 HashTable 中删除一个 key,返回被删除的这个 Value
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 遍历 HashTable，返回所有 kv pair（这个接口不好）
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    /// 遍历 HashTable，返回 kv pair 的 Iterator
    /// ---
    /// 1. 拿到一个关于某个 table 下的拥有所有权的Iterator \
    /// 2. 对 Iterator 做 map \
    /// 3. 将 map 出来的每个 item 转换成 Kvpair，建议实现 Into<Kvpair>
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
    // // 有则返回，无则创建
    // fn get_or_create_table<T>(&self, table: &str) -> T;
}

/// 提供 Storage iterator，这样 trait 的实现者只需要
/// 把它们的 iterator 提供给 StorageIter，然后它们保证
/// next() 传出的类型实现了 Into<Kvpair> 即可
pub struct StorageIter<T> {
    data: T,
}

impl<T> StorageIter<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Iterator for StorageIter<T>
where
    T: Iterator,
    T::Item: Into<Kvpair>,
{
    type Item = Kvpair;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|v| v.into())
    }
}
