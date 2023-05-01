use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    title: String,
    #[serde(rename = "type")]
    type_: String,
    properties: Option<Properties>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "firstName")]
    first_name: FirstName,
    #[serde(rename = "lastName")]
    last_name: LastName,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirstName {
    #[serde(rename = "type")]
    type_: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastName {
    #[serde(rename = "type")]
    type_: String,
    description: String,
}

pub struct St {
    // structure name
    name: String,
    // a list of structure fields
    fields: Vec<Fd>,
}

pub struct Fd {
    name: String,
    type_: String,
}

impl St {
    pub fn new(name: impl Into<String>, fields: Vec<Fd>) -> Self {
        Self {
            name: name.into(),
            fields,
        }
    }
}

impl Fd {
    pub fn new(name: impl Into<String>, ty: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            type_: ty.into(),
        }
    }
}

// impl From<Schema> for St {
//     fn from(value: Schema) -> Self {
//         todo!()
//     }
// }

impl Schema {
    pub fn into_st(&self) -> St {
        St {
            name: self.title.clone(),
            fields: todo!(),
        }
    }
}
