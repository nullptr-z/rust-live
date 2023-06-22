use anyhow::{anyhow, Result};
use async_trait::async_trait;
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
    match typed {
        "file" => FileFetcher(source).fetch().await,
        // support http/https
        "http" => UrlFetcher(source).fetch().await,
        _ => Err(anyhow!("not support yet: {typed}")),
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
        Ok(fs::read_to_string(&self.0[7..]).await?)
    }
}
