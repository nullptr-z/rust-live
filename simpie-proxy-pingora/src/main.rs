use anyhow::Result;
use pingora::{proxy::http_proxy_service, server::Server};
use simpie_proxy_pingora::SimpProxy;
use tracing::info;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut server = Server::new(None)?;
    server.bootstrap();

    let proxy_addr = "0.0.0.0:8080";
    let sp: SimpProxy = SimpProxy {};
    let mut service = http_proxy_service(&server.configuration, sp);
    service.add_tcp(proxy_addr);
    server.add_service(service);

    info!("proxy running on: {}", proxy_addr);
    server.run_forever();

    // Ok(()) PASS
}
