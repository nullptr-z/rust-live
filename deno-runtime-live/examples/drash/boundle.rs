use std::path::Path;

use deno_bundler::{BundleOptionsBuilder, BundleType};
use deno_core::resolve_url_or_path;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let options = BundleOptionsBuilder::default()
        .bundle_type(BundleType::Classic)
        .build()
        .unwrap();
    let f = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/drash/app.ts");
    let f = f.to_string_lossy().to_string();
    println!("【 f 】==> {:?}", f);
    let m = resolve_url_or_path(&f).unwrap();
    let (bundle, _) = deno_bundler::bundle(m, options).await.unwrap();

    println!("{}", bundle);
}
