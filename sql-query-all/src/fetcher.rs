use anyhow::{anyhow, Result};
use async_trait::async_trait;
use std::path::Path;
use tokio::fs;

// 任何数据源获取方式都必须实现Fetch
#[async_trait]
pub trait Fetcher {
    type Error;
    async fn fetch(&self) -> Result<String, Self::Error>;
}

pub async fn retrieve_data(source: impl Into<String>) -> Result<String> {
    let source = source.into();

    let typed = &source[..4];
    println!("【 typed 】==> {:?}", typed);
    match typed {
        "file" => FileFetcher(source).fetch().await,
        // support http/https
        "http" => UrlFetcher(source).fetch().await,
        // fetcher default the local file
        _ => {
            let file = get_current_path(source.as_str());
            FileFetcher(file).fetch().await
        }
    }
}

// 通过url方式请求数据
struct UrlFetcher(pub(crate) String);
// 读取本地文件的数据
struct FileFetcher(pub(crate) String);

#[async_trait]
impl Fetcher for UrlFetcher {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        let req = reqwest::get(self.0.as_str()).await?.text().await?;
        Ok(req)
    }
}

#[async_trait]
impl Fetcher for FileFetcher {
    type Error = anyhow::Error;
    async fn fetch(&self) -> Result<String, Self::Error> {
        println!("【 file fetch 】");
        Ok(fs::read_to_string(&self.0[7..]).await?)
    }
}

// 加载本地文件
pub fn get_current_path(path: &str) -> String {
    // 使用给定的相对路径创建一个 Path 对象
    let path = Path::new(path);

    // 使用 canonicalize 函数转换为绝对路径
    let absolute_path = std::fs::canonicalize(path).expect("Failed to get absolute path");

    // 处理平台差异
    let file_protocol_path = if cfg!(target_os = "windows") {
        format!("file:///{}/", absolute_path.display())
    } else {
        format!("file://{}", absolute_path.display())
    };

    return file_protocol_path;
}
