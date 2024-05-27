use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
    address: Vec<String>,
}

fn main() {
    let user = User {
        name: "John Doe".to_string(),
        age: 30,
        address: vec!["123 Main St".to_string(), "Anytown, USA".to_string()],
    };

    let json = serde_json::to_string(&user).unwrap();
    println!("JSON: {}", json);

    let user1: User = serde_json::from_str(&json).unwrap();

    assert_eq!(user, user1)
}
