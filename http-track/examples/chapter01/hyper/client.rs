use http_body_util::{BodyExt, Empty};
use hyper::{Request, body::Bytes};
use hyper_util::client::legacy::Client;
use hyper_util::rt::TokioExecutor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建客户端
    let client = Client::builder(TokioExecutor::new()).build_http();

    // 构建请求
    let req = Request::builder()
        .uri("http://127.0.0.1:3000/abc")
        .body(Empty::<Bytes>::new())?;

    // 发送请求
    let res = client.request(req).await?;

    println!("Status: {}", res.status());

    // 读取 body
    let body = res.collect().await?.to_bytes();
    println!("Body: {}", String::from_utf8_lossy(&body));

    Ok(())
}
