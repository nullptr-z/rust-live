use deno_core::{
    anyhow::{Ok, Result},
    resolve_url_or_path,
    serde::{de::DeserializeOwned, Deserialize},
    serde_v8, v8, JsRuntime, RuntimeOptions,
};

#[tokio::main]
async fn main() -> Result<()> {
    let options = Default::default();
    let mut rt = JsRuntime::new(options);

    let path = format!("{}basic_module.js", env!("CARGO_MANIFEST_DIR"));
    let url = resolve_url_or_path(&path)?;
    let id = rt.load_main_module(&url, None).await?;
    rt.mod_evaluate(id).await??;
    rt.run_event_loop(false).await?;

    // let code = include_str!("basic.js");
    // let ret: String = eval(&mut rt, code).await?;
    // print!("Rust: {ret:?}");

    Ok(())
}

#[allow(dead_code)]
async fn eval<T>(rt: &mut JsRuntime, code: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    // let code = include_str!("basic.js");
    let ret = rt.execute_script("<anon>", code)?;
    let result = rt.resolve_value(ret).await?;
    let scope = &mut rt.handle_scope();
    let result = v8::Local::new(scope, result);
    Ok(serde_v8::from_v8(scope, result)?)
}
