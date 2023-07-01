use reqwest::StatusCode;

use crate::{
    pb::{Hget, Hset},
    CommandResponse, CommandServer, Kvpair, Value,
};

fn verification_table<'a>(table: &'a str, res: &mut CommandResponse) -> Option<&'a str> {
    if table.is_empty() {
        res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _;
        res.message = format!("Not found Database Table: {}", table);
        return None;
    }
    Some(table)
}

impl CommandServer for Hget {
    fn execute<T>(self, store: &impl crate::Storage) -> CommandResponse {
        let mut res = CommandResponse::default();

        let table: Option<&str> = verification_table(&self.table, &mut res);
        if table.is_none() {
            return res;
        };

        if let Ok(Some(result)) = store.get(table.unwrap(), &self.key) {
            res.status = StatusCode::OK.as_u16() as _;
            res.values.push(result.clone());
            // res.pairs.push(Kvpair {
            //     key: self.key.to_string(),
            //     value: Some(result),
            // });
        } else {
            println!(
                "【  StatusCode::INTERNAL_SERVER_ERROR 】==> {:?}",
                StatusCode::INTERNAL_SERVER_ERROR
            );
            res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _;

            res.message = format!("Fialed go get field value for key: {}", self.key);
        }

        res
    }
}

impl CommandServer for Hset {
    fn execute<T>(self, store: &impl crate::Storage) -> CommandResponse {
        let mut res = CommandResponse::default();

        let table = verification_table(&self.table, &mut res);
        if table.is_none() {
            return res;
        };

        if let Some(Kvpair { key, value }) = self.pair {
            let value = value.map_or(Value::default(), |v| v);

            let result = store.set(table.unwrap(), key.clone(), value);

            if let Ok(v) = result {
                res.status = StatusCode::OK.as_u16() as _;
                res.message = "添加成功".to_owned();
                res.values.push(v.unwrap());
                // res.pairs.push(Kvpair { key, value: result });
            } else {
                res.status = StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _;
                res.message = format!(
                    "Failed to Add, key exists value: {}:{:?}",
                    key,
                    result.unwrap()
                );
            }
        };

        res
    }
}
