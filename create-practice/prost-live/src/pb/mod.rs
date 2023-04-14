mod prost_live;

use std::{collections::HashMap, option::IntoIter};

use bytes::Bytes;
pub use prost_live::*;

use self::person::{PhoneNumber, PhoneType};

impl Person {
    pub fn new(
        name: impl Into<String>,
        id: i32,
        email: impl Into<String>,
        phones: impl Into<Vec<PhoneNumber>>,
        data: impl IntoIterator<Item = u8>,
        scores: impl IntoIterator<Item = (String, i32)>,
    ) -> Person {
        Person {
            name: name.into(),
            id,
            email: email.into(),
            phones: phones.into(),
            data: data.into_iter().collect(),
            scores: scores.into_iter().collect(),
        }
    }
}

impl PhoneNumber {
    pub fn new(number: impl Into<String>, phone_type: PhoneType) -> Self {
        Self {
            number: number.into(),
            phone_type: phone_type.into(),
        }
    }
}
