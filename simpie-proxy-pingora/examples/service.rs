use axum::{
    Json, Router,
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    routing::{get, post},
};
use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 3001)]
    port: u16,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(users))
        .route("/create", post(create_user))
        .route_layer(middleware::from_fn(
            move |req: Request<Body>, next: Next| {
                let port = args.port;
                info!("the service is running on port {}", port);
                next.run(req)
            },
        ));

    let addr = format!("0.0.0.0:{}", args.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!("listening on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "welcome to use pingora!"
}

async fn users() -> (StatusCode, Json<Vec<User>>) {
    let users = vec![User {
        id: 0,
        username: "zhou".to_string(),
    }];

    (StatusCode::OK, Json(users))
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
