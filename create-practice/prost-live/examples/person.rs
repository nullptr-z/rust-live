use std::collections::HashMap;

use prost::Message;
use prost_live::pb::{
    person::{PhoneNumber, PhoneType},
    Person,
};

fn main() {
    let phones = vec![PhoneNumber::new("123-456789", PhoneType::Mobile)];
    let data = vec![1, 2, 3, 4, 5];
    let scores = vec![("math".to_string(), 100), ("english".to_string(), 99)];
    let person = Person::new(
        "zhou zheng",
        101,
        "zhengmr0646@gmail.com",
        phones,
        data,
        scores,
    );

    ////////////////////////// 1. encode_to_vec() //////////////////////////
    let v1 = person.encode_to_vec();
    let v2 = person.encode_length_delimited_to_vec(); // 第一个元素是长度，后面是内容与encode_to_vec()一样

    ////////////////////////// 2. decode_to_vec() //////////////////////////
    // decode_to_vec() 与 encode_to_vec() 对应
    let person: Person = Person::decode(v1.as_slice()).unwrap();
    println!("{person:#?}");

    ////////////////////////// 3. decode_length_delimited_to_vec() //////////////////////////
    // decode_length_delimited_to_vec() 与 encode_length_delimited_to_vec() 对应
    let person: Person = Person::decode_length_delimited(v2.as_slice()).unwrap();
    println!("{person:#?}");

    // 将Rust结构体转换为JSON字符串
    let json = serde_json::to_string(&person).unwrap();
    println!("{json:#?}");
}
