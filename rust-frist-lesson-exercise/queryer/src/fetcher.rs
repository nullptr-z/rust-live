// Rust 的 async trait 还没有稳定，可以用 async_trait 宏
use anyhow::{anyhow, Ok, Result};
use async_trait::async_trait;
use tokio::fs;

#[async_trait]
pub trait Fetch {
  type Error;
  async fn fetch(&self) -> Result<String, Self::Error>;
}

// 从文件源或者 http 源中获取数据，组成 data frame
pub async fn retrieve_data(source: impl AsRef<str>) -> Result<String> {
  let name = source.as_ref();
  match &name[..4] {
    // 包括 http / https
    "http" => UrlFetcher(name).fetch().await,
    // 处理 file://
    "file" => FileFetcher(name).fetch().await,
    _ => return Err(anyhow!("暂时只支持http/https/file数据源")),
  }
}

struct UrlFetcher<'a>(pub(crate) &'a str);
struct FileFetcher<'a>(pub(crate) &'a str);

#[async_trait]
impl<'a> Fetch for UrlFetcher<'a> {
  type Error = anyhow::Error;

  async fn fetch(&self) -> Result<String, Self::Error> {
    Ok(reqwest::get(self.0).await?.text().await?)
  }
}

#[async_trait]
impl<'a> Fetch for FileFetcher<'a> {
  type Error = anyhow::Error;

  async fn fetch(&self) -> Result<String, Self::Error> {
    Ok(fs::read_to_string(&self.0[7..]).await?)
  }
}
