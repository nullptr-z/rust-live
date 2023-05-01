mod json_schema;

use proc_macro::TokenStream;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    println!("【 input 】==> {:#?}", input);
    "fn hello() { println!(\"hello world\"); }".parse().unwrap()
}

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    println!("【 generate input 】==> {:#?}", input);

    Default::default()
}
