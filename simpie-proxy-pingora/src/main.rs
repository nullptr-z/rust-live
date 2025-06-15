mod conf;

use anyhow::Result;
use clap::Parser;
use pingora::{proxy::http_proxy_service, server::Server};
use simpie_proxy_pingora::{SimpProxy, conf::ProxyConfig};
use std::path::PathBuf;
use tracing::info;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short)]
    config: PathBuf,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let mut server = Server::new(None)?;
    server.bootstrap();

    let proxy_config = ProxyConfig::load(args.config)?;
    let sp = SimpProxy::new(proxy_config);
    let port = sp.config().load().global.port;
    let proxy_addr = &format!("0.0.0.0:{}", port);

    let mut service = http_proxy_service(&server.configuration, sp);
    service.add_tcp(proxy_addr);
    server.add_service(service);
    info!("proxy running on: {}", proxy_addr);
    server.run_forever();

    // Ok(()) PASS
}
