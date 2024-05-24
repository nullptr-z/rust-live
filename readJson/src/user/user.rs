use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub avatar: String,
    pub role: String,
    pub status: i64,
}
