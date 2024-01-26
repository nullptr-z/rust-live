fn main() {
    tonic_build::configure()
        .out_dir("src/pb/")
        .compile(&["src/pb/broadcast.proto"], &[""])
        .unwrap();

    print!("cargo:rerun-if-change=src/pb/broadcast.proto");
    print!("cargo:rerun-if-change=build.rs");
}
