use crate::{
    error::KvError,
    pb::abi::{Kvpair, Value},
    Storage,
};
use sled::{Db, IVec};
use std::{ops::Deref, path::Path};

pub struct SledDB(Db);

impl SledDB {
    /// 读取本都数据库文件
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(sled::open(path).unwrap())
    }

    /// 在 sleddb 里，因为它可以 scan_prefix，我们用 prefix
    /// 来模拟一个 table。当然，还可以用其它方案。
    fn get_full_key(table: &str, key: &str) -> String {
        format!("{}:{}", table, key)
    }

    /// 遍历 table 的 key 时，我们直接把 prefix: 当成 table
    fn get_table_prefix(table: &str) -> String {
        format!("{}:", table)
    }
}

impl Storage for SledDB {
    fn get(
        &self,
        table: impl Into<String>,
        key: impl Into<String>,
    ) -> Result<Option<Value>, KvError> {
        let key = SledDB::get_full_key(&table.into(), &key.into());
        self.deref().get(key).flip()
    }

    fn set(
        &self,
        table: impl Into<String>,
        key: impl Into<String>,
        value: Value,
    ) -> Result<Option<Value>, KvError> {
        let mut key = SledDB::get_full_key(&table.into(), &key.into());
        // let sled = self.deref().insert(key.to_string(), value.into());

        Ok(Some(value))
    }

    fn contains(&self, table: impl Into<String>, key: impl Into<String>) -> Result<bool, KvError> {
        todo!()
    }

    fn del(
        &self,
        table: impl Into<String>,
        key: impl Into<String>,
    ) -> Result<Option<Value>, KvError> {
        todo!()
    }

    fn get_all(&self, table: impl Into<String>) -> Result<Vec<Kvpair>, KvError> {
        todo!()
    }

    fn get_iter(
        &self,
        table: impl Into<String>,
    ) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        todo!()
    }
}

trait SledResult<T, E> {
    fn flip(self) -> Result<Option<Value>, KvError>;
}

impl<T, E> SledResult<T, E> for Result<Option<T>, E>
where
    T: Deref<Target = [u8]>,
{
    fn flip(self) -> Result<Option<Value>, KvError> {
        match self {
            Ok(value) => Ok(value.map(|m| m.as_ref().into())),
            Err(_) => Err(KvError::Internal("error flipr".to_owned())),
        }
    }
}

impl From<Value> for IVec {
    fn from(value: Value) -> Self {
        match value.value {
            Some(v) => match v {
                crate::pb::abi::value::Value::String(s) => s.as_str().into(),
                _ => todo!(),
            },
            None => todo!(),
        }
    }
}

impl std::ops::Deref for SledDB {
    type Target = Db;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
