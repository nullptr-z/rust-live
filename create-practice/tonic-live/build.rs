fn main() {
    println!("cargo:rerun-if-changed=abi.proto");
    println!("cargo:rerun-if-changed=build.rs");

    tonic_build::configure()
        .out_dir("src/pb")
        .compile(&["./abi.proto"], &["./"])
        .unwrap();
}
