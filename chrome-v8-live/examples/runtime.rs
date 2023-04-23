use chrome_v8_live::JsRuntime;

fn main() {
    JsRuntime::init();
    // new Isolate
    let mut runtime = JsRuntime::new(Default::default());

    let code = r#"
        function hello(){
            return {r1:"Hello", r2: "World"}
        }
        hello();
      "#;
    let result = runtime.execute_script(code);
    println!("【 result 】==> {:?}", result);
}
