mod user;

use crate::user::user::User;

fn main() {
    let user: User = User {
        id: 1000,
        name: "zz".into(),
        email: "zhengmr@zz.com".into(),
        phone: "12345678901".into(),
        avatar: "URL_ADDRESS".into(),
        role: "admin".into(),
        status: 1,
    };

    println!("{}", serde_json::to_string(&user).unwrap());
}
