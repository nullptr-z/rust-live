use dashmap::{mapref::one::Ref, DashMap};

use crate::{KvError, Kvpair, Storage, Value};
use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Deref, DerefMut},
};

#[derive(Debug, Default)]
pub struct MemoryDB(DashMap<String, DashMap<String, Value>>);

impl MemoryDB {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Storage for MemoryDB {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table::<DashMap<String, DashMap<String, Value>>>(table);
        let value: Option<Value> = table.get(key).map_or(None, |v| v.value().clone());

        Ok(value)
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table::<DashMap<String, DashMap<String, Value>>>(table);
        let result = table.insert(key, value);

        Ok(result)
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, Value> {
        self.0.borrow().contains_key(key);
        todo!()
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        todo!()
    }

    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        todo!()
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        todo!()
    }

    fn get_or_create_table<T>(&self, table: &str) -> T {
        let table = self.0.borrow().get(table);
        let aaa = table.unwrap().value();
        match table {
            Some(table) => table.value().clone(),
            None => MemoryDB::new().0,
        }
    }
}

impl Deref for MemoryDB {
    type Target = DashMap<String, DashMap<String, Value>>;

    fn deref(&self) -> &Self::Target {
        &self.0.borrow()
    }
}

impl DerefMut for MemoryDB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.borrow_mut()
    }
}
