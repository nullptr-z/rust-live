use sled::{Db, IVec};

use crate::{error, Storage};

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

// impl From<crate::Value> for IVec {
//     // type Error = error::KvError;

//     fn from(value: crate::Value) -> Self {
//         let val = if let Some(v) = value.value {
//             match v {
//                 crate::pb::value::Value::String(val) => IVec::from(val.as_str()),
//                 crate::pb::value::Value::Binary(val) => IVec::from_iter(val.into_iter()),
//                 crate::pb::value::Value::Integer(val) => IVec::from(val.to_string().as_str()),
//                 crate::pb::value::Value::Float(val) => IVec::from(val.to_string().as_str()),
//                 crate::pb::value::Value::Bool(val) => IVec::from(val.to_string().as_str()),
//             }
//         } else {
//             IVec::default()
//         };

//         val
//     }
// }

impl Storage for SledDB {
    fn get(&self, table: &str, key: &str) -> Result<Option<crate::Value>, crate::KvError> {
        todo!()
    }

    fn set(
        &self,
        table: &str,
        key: String,
        value: crate::Value,
    ) -> Result<Option<crate::Value>, crate::KvError> {
        let table_key = SledDB::get_full_key(table, &key);
        let data:Vec<u8>=value.map(|v|v.as);
        let ins = self.0.insert(table_key, value);
        todo!()
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
