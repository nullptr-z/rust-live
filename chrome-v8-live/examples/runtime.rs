use chrome_v8_live::JsRuntime;

fn main() {
    JsRuntime::init();
    JsRuntime::init();
    // new Isolate
    let mut runtime = JsRuntime::new(None);

    let code = r#"
        function hello(){
            print({a:"Hello ",b:"World",c:"我的天那"});
            return fetch("https://www.rust-lang.org/");
        }
        hello();
      "#;
    let result = runtime.execute_script(code, true);
    println!("\n\nexecute_script result: {:?}", result);
}
