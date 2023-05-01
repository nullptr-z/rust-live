mod json_schema;
use crate::json_schema::Schema;

pub mod generate {
    use proc_macros::generate;

    generate!("proc-macros/fixture/person.json");
}

// use generate::*;

fn main() {
    let schema: Schema = serde_json::from_str(include_str!("../fixtures/person.json")).unwrap();
    println!("【 schema 】==> {:#?}", schema);
}
