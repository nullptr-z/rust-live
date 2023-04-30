use std::{fs, path::Path};

use deno_core::{JsRuntime, RuntimeOptions};
use zstd::encode_all;

const SNAPSHOT_FILE: &str = "snapshots/main.bin";

fn main() {
    let options = RuntimeOptions {
        will_snapshot: true,
        ..Default::default()
    };
    let mut rt = JsRuntime::new(options);
    // 保存快照的文件路径
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(SNAPSHOT_FILE);
    // 创建快照
    let data = rt.snapshot();
    // 压缩的 encode_all 方法需要，快照数据转换成`&[u8]`；注释： * 调用 DeRef 得到 [u8]：
    let data = &*data;
    // 对快照数据压缩
    let compressed = encode_all(data, 7).unwrap();
    // 将快照数据写入文件，write的 write 接受的是一个 AsRef<[u8]> 类型的数据,这里的 data 是 StartupData 类型，实现了AsRef<[u8]>，所以可以直接传入
    fs::write(filename, compressed).unwrap();
}
