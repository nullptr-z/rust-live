fn main() {
    tonic_build::configure()
        // .type_attribute(
        //     ".",
        //     "#[derive[Hash,Eq,serde::Serialize,serde::DeSerialize]]",
        // )
        .out_dir("src/abi/")
        .compile(&["src/abi/message.proto"], &[""])
        .unwrap();

    print!("cargo:rerun-if-change=src/abi/message.proto");
    print!("cargo:rerun-if-change=build.rs");
}
