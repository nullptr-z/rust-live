use axum::{middleware::AddExtension, routing::get, Router, Server};
use axum_ws_live::{ws_handler, ChatState};
use std::net::SocketAddr;
use tower::limit::ConcurrencyLimitLayer;

#[tokio::main]
async fn main() {
    todo!()
}
// async fn main() {
//     let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
//     let app = Router::new().route(
//         "/ws",
//         get(ws_handler).layer(ConcurrencyLimitLayer::new(ChatState::new())),
//     );

//     Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
