use async_trait::async_trait;
use pingora::{http::ResponseHeader, prelude::*};
use tracing::info;

pub struct SimpProxy {}

#[async_trait]
impl ProxyHttp for SimpProxy {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let peer = HttpPeer::new("127.0.0.1:3000", false, "localhost".to_owned());

        info!("upstream peer: {}", peer.to_string());
        Ok(Box::new(peer))
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()>
    where
        Self::CTX: Send + Sync,
    {
        upstream_request.append_header("x-token", "hi hi")?;

        info!("-------request filter-------");
        Ok(())
    }

    fn upstream_response_filter(
        &self,
        _session: &mut Session,
        upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_response.append_header("my-server", "zz/service")?;

        info!("-------response filter-------");
        Ok(())
    }
}
