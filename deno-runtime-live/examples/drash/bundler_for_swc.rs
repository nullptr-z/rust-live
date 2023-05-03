use swc::{
    config::{Config, ModuleConfig},
    Compiler,
};

fn main() {
    let config = Config::new().with_module(ModuleConfig::default());
    let compiler = Compiler::new(config);

    let input = r#"console.log('Hello, SWC!')"#;
    let output = compiler
        .process_js(input, None, None)
        .expect("failed to process JS code");

    println!("{}", output.code);
}

#[cfg(test)]
mod test {
    fn te() {
        main()
    }
}
