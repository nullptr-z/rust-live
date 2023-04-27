use deno_core::{
    anyhow::{Ok, Result},
    serde_v8, v8, JsRuntime,
};

#[tokio::main]
async fn main() -> Result<()> {
    let options = Default::default();
    let mut rt = JsRuntime::new(options);
    let code = include_str!("basic.js");
    let ret = rt.execute_script("<anon>", code)?;
    let result = rt.resolve_value(ret).await?;
    let scope = &mut rt.handle_scope();
    let result = v8::Local::new(scope, result);
    let result: String = serde_v8::from_v8(scope, result)?;
    println!(" {result:?}");

    Ok(())
}
