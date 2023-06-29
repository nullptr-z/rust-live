use crate::{error, KvError, Kvpair, Storage, Value};
use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    ops::{Deref, DerefMut},
};

pub struct MemoryDB(RefCell<HashMap<String, Value>>);

impl MemoryDB {
    pub fn new() -> Self {
        Self(RefCell::new(HashMap::<String, Value>::with_capacity(10)))
    }
}

impl Storage for MemoryDB {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let value = self
            .borrow_mut()
            .get(key)
            .map_or(None, |v| Some(v.to_owned()));

        Ok(value)
    }

    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let result = self.borrow_mut().insert(key, value);

        Ok(result)
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, Value> {
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
}

impl Deref for MemoryDB {
    type Target = RefCell<HashMap<String, Value>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MemoryDB {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
