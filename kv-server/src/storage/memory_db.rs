use dashmap::{mapref::one::Ref, DashMap};

use crate::{KvError, Kvpair, Storage, Value};
use std::{
    borrow::Borrow,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Default)]
pub struct MemoryDB(DashMap<String, DashMap<String, Value>>);

impl MemoryDB {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_or_create_table(&self, table_name: &str) -> Ref<String, DashMap<String, Value>> {
        let table = self.0.borrow().get(table_name);
        match table {
            Some(table) => table,
            None => {
                let entry = self.0.entry(table_name.to_owned()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for MemoryDB {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        let value: Option<Value> = table.get(key).map_or(None, |v| Some(v.value().clone()));

        Ok(value)
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        let result = table.insert(key, value);

        Ok(result)
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, Value> {
        let table = self.get_or_create_table(table);
        Ok(table.contains_key(key))
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.remove(key).map(|(_k, v)| v))
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        let table = self.get_or_create_table(table);
        let kvs = table
            .iter()
            .map(|v| Kvpair::new(v.key(), v.value().clone()))
            .collect();

        Ok(kvs)
    }

    /// dyn Iterator 忽略其他，只关心是否可以使用 next()
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        // 使用 clone() 来获取 table 的 snapshot, 高效的，除非hash表很大
        let table = self.get_or_create_table(table).clone();
        let iter = crate::StorageIter::new(table.into_iter()); // 这行改掉了
        Ok(Box::new(iter))
    }
}

impl From<(String, Value)> for Kvpair {
    fn from(item: (String, Value)) -> Self {
        Self {
            key: item.0,
            value: Some(item.1),
        }
    }
}

impl Deref for MemoryDB {
    type Target = DashMap<String, DashMap<String, Value>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MemoryDB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use crate::MemoryDB;

    #[test]
    fn test_get_or_create_table() {}
}
