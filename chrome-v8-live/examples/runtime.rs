use chrome_v8_live::JsRuntime;

fn main() {
    JsRuntime::init();
    // new Isolate
    let mut runtime = JsRuntime::new(Default::default());

    let code = r#"
        function hello(){
            print("Hello ","World","我的天那");
            return {r1:"Hello", r2: "World"}
        }
        hello();
      "#;
    let result = runtime.execute_script(code);
    println!("\n\nexecute_script result: {:?}", result);
}
