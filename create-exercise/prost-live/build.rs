use anyhow::Result;
use prost_build::Config;

/**
 * prost::Message 默认实现 Default Trait
 */
fn main() -> Result<()> {
    // 指定哪些文件改变重新编译
    println!("cargo:rerun-if-changed=person.proto");
    println!("cargo:rerun-if-changed=build.rs");
    // 编译 proto 文件
    Config::new()
        // .include_file("build.rs")
        .out_dir("src/pb") // 指定OUT_DIR
        .compile_protos(&["person.proto"], &["."])?;
    Ok(())
}
