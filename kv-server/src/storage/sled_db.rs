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

struct SledResult<T, E> {
    inner: Option<Result<T, E>>,
}

impl<T, E> Into<SledResult<T, E>> for Option<Result<T, E>> {
    fn into(self) -> SledResult<T, E> {
        SledResult { inner: self }
    }
}

impl<T, E> SledResult<T, E> {
    fn flip(self) -> Result<Option<T>, E> {
        self.inner.map_or(Ok(None), |v| v.map(|v| Some(v)))
    }
}

fn flip<T, E>(v: Option<Result<T, E>>) -> Result<Option<T>, E> {
    v.map_or(Ok(None), |v| v.map(|v| Some(v)))
}

impl Storage for SledDB {
    // fn get(&self, table: &str, key: &str) -> Result<Option<crate::Value>, crate::KvError> {
    //     let table_key = SledDB::get_full_key(table, &key);
    //     let result = self.0.get(table_key)?.map(|val| val.as_ref().try_into());

    //     flip(result)
    // }

    fn get(&self, table: &str, key: &str) -> Result<Option<crate::Value>, crate::KvError> {
        let table_key = SledDB::get_full_key(table, &key);
        let result = self.0.get(table_key)?.map(|val| val.as_ref().try_into());

        // TODO 使用SledResult简化, Result<Option<IVec>, Error> to Result<Option<crate::Value>, crate::KvError>

        let sr = Into::<SledResult<crate::Value, KvError>>::into(result);
        sr.flip()
    }

    fn set(
        &self,
        table: &str,
        key: String,
        value: crate::Value,
    ) -> Result<Option<crate::Value>, crate::KvError> {
        let table_key = SledDB::get_full_key(table, &key);
        let result = self
            .0
            .insert(&table_key, IVec::try_from(value)?)?
            .map(|val| {
                let vv = val.as_ref();
                let vvv = vv.try_into();
                vvv
            });

        flip(result)
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, crate::Value> {
        todo!()
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<crate::Value>, crate::KvError> {
        todo!()
    }

    fn get_all(&self, table: &str) -> Result<Vec<crate::Kvpair>, crate::KvError> {
        todo!()
    }

    fn get_iter(
        &self,
        table: &str,
    ) -> Result<Box<dyn Iterator<Item = crate::Kvpair>>, crate::KvError> {
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
            .set("sled_db1", "sled_key1".into(), "sled db value 1".into())
            .unwrap();

        let value = sled_db.get("sled_db1", "sled_key1".into());

        println!("【 value 】==> {:?}", value);
        assert!(value.unwrap().is_some())
    }
}
