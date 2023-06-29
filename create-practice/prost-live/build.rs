use anyhow::Result;
use prost_build::Config;

/**
 * prost::Message 默认实现 Default Trait
 */
fn main() -> Result<()> {
    // 指定哪些文件改变时重新编译
    println!("cargo:rerun-if-changed=person.proto");
    println!("cargo:rerun-if-changed=build.rs");

    // 编译 proto 文件
    Config::new()
        // 指定OUT_DIR
        .out_dir("src/pb")
        // proto中定义为bytes的字段，编译为rust中Bytes类型
        // . 表示对所有的域都是使用,也可以指定为某一个层级的域
        // @1:不支持serde::Serialize,注释掉避免报错
        // .bytes(&["."])
        // --------------------------------------------------------
        // proto中定义为map的字段，编译为rust中HashMap类型
        // 这里指定的是scores字段
        .btree_map(&["scores"])
        // 为data字段添加属性; 同样`"."`表示对所有的域都是使用
        // 这里 skip_serializing_if = "Vec::is_empty" 表示如果data字段为空，则不序列化
        // 这里 default 表示如果data字段为空，则使用默认值
        .field_attribute(
            "data",
            r#"#[serde(skip_serializing_if = "Vec::is_empty", default)]"#,
        )
        // 给指定结构加上属性，同样的表示对所有的域都是使用
        // 注意事项：
        // 1、这里的属性是在结构体上加上，而不是在字段上加上
        // 2、给定属性时要给完整的路径，比如这里的serde::Serialize
        // 3、如果结构的某个字段不支持给定属性，那么编译会报错
        // 3.1、比如这里的AddressBook结构，它的data被指定编译[@1]为Bytes类型，而Bytes类型不支持serde::Serialize
        .type_attribute(&".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["person.proto"], &["."])?;
    Ok(())
}

// 简化使用Prost的库create: prost-build-config
