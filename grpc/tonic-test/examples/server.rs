use social_engineer::server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    server::start().await;
}
