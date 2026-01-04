//! Headers 处理示例
//!
//! 运行: cargo run --example hy-headers
//! 测试: curl -H 'X-Custom: hello' http://127.0.0.1:3000

use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::header::{CONTENT_TYPE, HeaderValue, USER_AGENT};
use hyper::{Method, StatusCode};
// TODO: 从 hyper::header 导入常量 CONTENT_TYPE, USER_AGENT
use hyper::{Request, Response, body::Incoming, server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: 绑定 TcpListener 到 127.0.0.1:3000
    let server = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is running on http://0.0.0.0:3000");

    loop {
        let (stream, _) = server.accept().await?;
        let io = TokioIo::new(stream);
        let service = service_fn(handle_controller);
        http1::Builder::new().serve_connection(io, service).await?;
    }
}

async fn handle_controller(req: Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error> {
    // ===== 读取请求基本信息 =====
    // 用到: req.method(), req.uri(), req.version()
    let method = req.method();
    let uri = req.uri();
    let version = req.version();
    println!(
        "Method: {:?}, URI: {:?}, Version: {:?}",
        method, uri, version
    );

    // ===== 读取所有 Headers =====
    // 用到: req.headers() -> &HeaderMap
    // 遍历: for (name, value) in req.headers() { ... }
    let headers = req.headers();
    println!("Request Headers:--------------------------------");
    for (name, value) in headers {
        println!("{:?}, Value: {:?}", name, value);
    }

    // ===== 读取特定 Header =====
    // 方式1 - 使用常量: req.headers().get(USER_AGENT)
    // 方式2 - 使用字符串: req.headers().get("x-custom")
    // 返回: Option<&HeaderValue>
    // 转字符串: value.to_str().ok()
    let user_agent = req.headers().get(USER_AGENT);
    println!("User-Agent: {:?}", user_agent);
    println!("------------------------------------------------");

    if method == &Method::POST {
        return Ok(handle_request_post(req).await?);
    }

    // ===== 构建响应并设置 Headers =====
    // 用到: Response::builder()
    //       .status(200)
    //       .header(CONTENT_TYPE, "text/plain")  // 常量方式
    //       .header("X-Custom", "value")         // 字符串方式
    //       .body(Full::new(Bytes::from("...")))
    println!("Response Headers:--------------------------------");
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "text/plain")
        .header("X-Custom", "value")
        .body(Full::new(Bytes::from("received a GET request")))
        .unwrap();
    println!("Response: {:?}", response);
    for (name, value) in response.headers() {
        println!("{:?}, Value: {:?}", name, value);
    }
    println!("Response status: {:?}", response.status());

    println!("------------------------ END REQUEST ------------------------\n\n");

    Ok(Response::new(response.into_body()))
}

async fn handle_request_post(
    req: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    // ===== 简单路由 =====
    // 用到: req.method() -> &Method
    //       req.uri().path() -> &str
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    if method != Method::POST {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Full::new(Bytes::from("Method not allowed")))
            .unwrap());
    }

    // ===== 读取 Content-Type =====
    // 用到: req.headers().get(CONTENT_TYPE)
    //       .and_then(|v| v.to_str().ok())
    let content_type = req.headers().get(CONTENT_TYPE);
    println!("Content-Type: {:?}", content_type);

    // ===== 读取请求 Body =====
    // 核心方法: req.collect().await?.to_bytes()
    // 返回: Bytes
    // 说明: Incoming 是流式的，collect() 收集全部数据
    // 转字符串: String::from_utf8_lossy(&body_bytes)
    let body = req.collect().await?.to_bytes();
    println!("Body: {:?}", String::from_utf8_lossy(&body));

    // ===== 返回不同状态码 =====
    // 用到: Response::builder().status(StatusCode::OK)
    //       Response::builder().status(StatusCode::NOT_FOUND)
    //       或直接用数字: .status(200), .status(404)

    let (resp_msg, status_code) = match (&method, path.as_str()) {
        (&Method::POST, "/post") => ("received a POST request", StatusCode::OK),
        _ => ("not found 404", StatusCode::NOT_FOUND),
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Full::new(Bytes::from(resp_msg)))
        .unwrap())
}
