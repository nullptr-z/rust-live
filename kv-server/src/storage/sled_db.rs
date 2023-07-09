use std::{convert, ops::Deref};

use sled::{Db, IVec};

use crate::{error, KvError, Storage};

#[derive(Debug)]
pub struct SledDB(Db);

impl Default for SledDB {
    fn default() -> Self {
        let db = sled::open("sled-db").unwrap();
        Self(db)
    }
}

impl SledDB {
    fn new(path: &str) -> Self {
        let db = sled::open(path).unwrap();
        Self(db)
    }

    // 完整table key
    fn get_full_key(table: &str, key: &str) -> String {
        format!("{}:{}", table, key)
    }
    // table 前缀
    fn get_table_prefix(table: &str) -> String {
        table.to_string()
    }
}

#[inline]
fn string_to_ivec<T>(v: T) -> Result<IVec, KvError>
where
    T: ToString,
{
    Ok(IVec::from(v.to_string().as_str()))
}

impl TryFrom<crate::Value> for IVec {
    type Error = error::KvError;

    fn try_from(value: crate::Value) -> Result<Self, Self::Error> {
        let val = if let Some(v) = value.value {
            match v {
                crate::pb::value::Value::String(val) => string_to_ivec(val),
                crate::pb::value::Value::Integer(val) => string_to_ivec(val),
                crate::pb::value::Value::Float(val) => string_to_ivec(val),
                crate::pb::value::Value::Bool(val) => string_to_ivec(val),
                crate::pb::value::Value::Binary(val) => Ok(IVec::from_iter(val.into_iter())),
            }
        } else {
            Err(KvError::ConvertError(value, "IVec"))
        };

        val
    }
}

pub trait SledOption<T, E> {
    fn flip(self) -> Result<Option<T>, E>;
}

impl<T, E> SledOption<T, E> for Option<Result<T, E>> {
    fn flip(self) -> Result<Option<T>, E> {
        self.map_or(Ok(None), |v| v.map(|v| Some(v)))
    }
}

pub trait SledResult<T, E> {
    fn flipr(self) -> Result<Option<crate::Value>, KvError>;
}

impl<T, E> SledResult<T, E> for Result<Option<T>, E>
where
    T: Deref<Target = [u8]>,
    // E: KvError,
    // for<'a> V: TryFrom<&'a [u8], Error = E>,
{
    fn flipr(self) -> Result<Option<crate::Value>, KvError> {
        match self {
            Ok(v) => v.map(|val| val.as_ref().try_into()).flip(),
            Err(_) => Err(KvError::Internal("error flipr".to_owned())),
        }
    }
}

impl Storage for SledDB {
    fn get(&self, table: &str, key: &str) -> Result<Option<crate::Value>, KvError> {
        let table_key = SledDB::get_full_key(table, &key);
        let result = self.0.get(table_key).flipr();

        result
    }

    fn set(
        &self,
        table: &str,
        key: String,
        value: crate::Value,
    ) -> Result<Option<crate::Value>, KvError> {
        let table_key = SledDB::get_full_key(table, &key);
        let result = self.0.insert(&table_key, IVec::try_from(value)?).flipr();

        result

        // let result = self
        //     .0
        //     .insert(&table_key, IVec::try_from(value)?)
        //     .unwrap()
        //     .map(|val| val.as_ref().try_into())
        //     .flip();
        // result
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, crate::Value> {
        todo!()
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<crate::Value>, KvError> {
        todo!()
    }

    fn get_all(&self, table: &str) -> Result<Vec<crate::Kvpair>, KvError> {
        todo!()
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = crate::Kvpair>>, KvError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{SledDB, Storage, Value};

    #[test]
    fn create_sled_db() {
        let sled_db = SledDB::default();

        sled_db
            .set("sled_db1", "sled_key1".into(), "sled db value 3".into())
            .unwrap();

        let value = sled_db.get("sled_db1", "sled_key1".into());

        println!("【 value 】==> {:?}", value);
        assert!(value.unwrap().is_some())
    }
}
