use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    response::Html,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, Server,
};
use jsonwebtoken as jwt;
use jwt::Validation;
use serde::{Deserialize, Serialize};
use std::{
    borrow::BorrowMut,
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{atomic::AtomicUsize, Arc, RwLock},
    time::SystemTime,
};

const SECRET: &[u8] = b"deadbeef";
// atomicUsize是一个原子类型，可以在多线程中安全的使用
static Next_Id: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    id: usize,
    user_id: usize,
    title: String,
    completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTodo {
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: usize,
    name: String,
    exp: usize,
}

#[derive(Debug, Default, Clone)]
struct TodoStore {
    items: Arc<RwLock<Vec<Todo>>>,
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    // these bounds are required by `async_trait`
    B: Send + 'static,
{
    type Rejection = HttpError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|e| {
                    println!("【 e 】==> {:?}", e);
                    HttpError::Auth
                })?;

        let key = jwt::DecodingKey::from_secret(SECRET);
        let token =
            jwt::decode(bearer.token(), &key, &Validation::default()).map_err(|e| {
                match e.kind() {
                    jwt::errors::ErrorKind::InvalidSignature => HttpError::InvalidSignature,
                    _ => HttpError::Auth,
                }
            })?;

        Ok(token.claims)
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum HttpError {
    Auth,
    Internal,
    InvalidSignature,
}

/// `type Rejection`要求实现 `IntoResponse Trait`
impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let (code, msg) = match self {
            HttpError::Auth => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            HttpError::InvalidSignature => (StatusCode::FORBIDDEN, "InvalidSignature`token失效"),
            HttpError::Internal => (StatusCode::INSUFFICIENT_STORAGE, "Internal Server Error"),
        };

        // 如果结构的每个字段都实现了`into_response`,那么这个结构就可以使用`into_response`
        (code, msg).into_response()
    }
}

#[tokio::main]
async fn main() {
    let store = init_store();

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/login", post(login_handler))
        .route(
            "/todos",
            get(todos_handler)
                .post(create_todo_handler)
                .layer(Extension(store)),
        );

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8000);
    println!("Listening on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn index_handler() -> Html<&'static str> {
    Html("the is axum live")
}

async fn todos_handler(
    claims: Claims,
    Extension(store): Extension<TodoStore>,
) -> Result<Json<Vec<Todo>>, HttpError> {
    match store.items.read() {
        Ok(guard) => {
            // let tt = guard.clone();
            // println!("【 guard 】==> {:?}", tt);
            Ok(Json(
                // filter过滤出当前用户的todo
                // map映射这些用户的clone,避免对其他用户clone
                guard
                    .iter()
                    .filter(|todo| todo.user_id == claims.id)
                    .map(|todo| todo.clone())
                    .collect(),
            ))
        }
        Err(_) => Err(HttpError::Internal),
    }
}

async fn create_todo_handler(
    claims: Claims,
    Json(param): Json<CreateTodo>,
    Extension(store): Extension<TodoStore>,
) -> Result<StatusCode, HttpError> {
    match store.items.write() {
        Ok(mut guard) => {
            let id = guard.len() as usize + 1;
            guard.push(Todo {
                id,
                // id: get_next_id(),
                user_id: claims.id,
                title: param.title,
                completed: false,
            });
            Ok(StatusCode::CREATED)
        }
        Err(_) => Err(HttpError::Internal),
    }
}

async fn login_handler(Json(_login): Json<LoginRequest>) -> Json<LoginResponse> {
    // skip login info validation`跳过登录信息验证;模拟登录成功
    let claims = Claims {
        id: 808,
        name: "zhengmr".to_string(),
        exp: get_epoch() + 14 * 24 * 60 * 60, // 14 天有效期的 token
    };
    let key = jwt::EncodingKey::from_secret(SECRET);
    let token = jwt::encode(&jwt::Header::default(), &claims, &key).unwrap();

    Json(LoginResponse { token })
}

fn get_next_id() -> usize {
    // fetch_add(@1, @2), @1为每次调用后对 Next_Id 的增量
    Next_Id.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

fn get_epoch() -> usize {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}

fn init_store() -> TodoStore {
    let mut store = TodoStore::default();
    store.items.write().unwrap().push(Todo {
        id: 1,
        user_id: 808,
        title: "todo1".to_string(),
        completed: false,
    });
    store.items.write().unwrap().push(Todo {
        id: 2,
        user_id: 808,
        title: "todo2".to_string(),
        completed: false,
    });
    store.items.write().unwrap().push(Todo {
        id: 3,
        user_id: 808,
        title: "todo3".to_string(),
        completed: false,
    });

    store
}
