fn main() {
    let mut config = prost_build::Config::new();

    config.bytes(&["."]);
    config.type_attribute(".", "#[derive(PartialOrd)]");
    config
        .out_dir("src/pb")
        .compile_protos(&["kv.proto"], &["."])
        .unwrap();

    // 指定哪些文件改变时重新编译
    println!("cargo:rerun-if-changed=kv.proto");
    println!("cargo:rerun-if-changed=build.rs");
}
