#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    broadcast::server::start().await;
}
