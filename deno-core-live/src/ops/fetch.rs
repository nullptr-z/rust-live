use deno_core::{
    error::AnyError,
    include_js_files, op,
    serde::{Deserialize, Serialize},
    ByteString, Extension, OpState, ZeroCopyBuf,
};
use reqwest::{Method, Url};
use std::{cell::RefCell, rc::Rc, str::FromStr};

#[derive(Deserialize)]
#[serde[rename_all="camelCase"]]
pub struct FetchArgs {
    method: String,
    url: String,
    headers: Vec<(ByteString, ByteString)>,
    body: Option<ZeroCopyBuf>,
}

#[derive(Serialize)]
pub struct FetchResponse {
    status: u16,
    status_text: String,
    headers: Vec<(ByteString, ByteString)>,
    body: Option<ZeroCopyBuf>,
}

// ops: @2
#[op]
async fn op_fetch(state: Rc<RefCell<OpState>>, args: FetchArgs) -> Result<FetchResponse, AnyError> {
    let state_ref = state.borrow();
    let client = state_ref.borrow::<reqwest::Client>().clone();
    // 将 Method 转换为大写
    let method = Method::from_str(&args.method.to_ascii_uppercase())?;
    // 将 string 的 url 转换为 Url 类型
    let url = Url::parse(&args.url)?;

    // 构建一个请求
    let mut req = client.request(method, url);
    // 设置Headers
    for (key, value) in args.headers {
        req = req.header(key.to_vec(), value.to_vec());
    }
    // 设置Body
    let req = if let Some(body) = args.body {
        req.body(body.to_vec())
    } else {
        req
    };
    // 发送请求，得到 response
    let res = req.send().await?;
    // 处理响应头
    let headers = res
        .headers()
        .iter()
        .map(|(key, value)| {
            (
                ByteString::from(key.as_str()),
                ByteString::from(value.as_bytes()),
            )
        })
        .collect();

    Ok(FetchResponse {
        status: res.status().as_u16(),
        status_text: res.status().canonical_reason().unwrap_or("").to_string(),
        headers,
        body: Some(ZeroCopyBuf::from(res.bytes().await?.to_vec())),
    })
}

// ops: @1
pub fn init() -> Extension {
    let file = include_js_files!(
        prefix ".",
        "fetch.js",
    );

    Extension::builder()
        .js(file)
        // ops 里可以注册一些自定义的函数
        .ops(vec![op_fetch::decl()])
        // state里面可以放一些全局的状态
        // 这里创建了 reqwest::Client 对象
        .state(move |state| {
            state.put::<reqwest::Client>(reqwest::Client::new());
            Ok(())
        })
        .build()
}