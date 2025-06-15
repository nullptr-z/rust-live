pub mod conf;

use async_trait::async_trait;
use axum::http::{self, StatusCode};
use conf::ProxyConfig;
use pingora::{http::ResponseHeader, prelude::*};
use tracing::info;

#[derive(Debug, Clone)]
pub struct SimpProxy {
    pub(crate) config: ProxyConfig,
}

pub struct ProxyContext {
    pub(crate) config: ProxyConfig,
}

impl SimpProxy {
    pub fn new(config: ProxyConfig) -> Self {
        Self { config }
    }

    pub fn config(&self) -> ProxyConfig {
        self.config.clone()
    }
}

#[async_trait]
impl ProxyHttp for SimpProxy {
    type CTX = ProxyContext;

    fn new_ctx(&self) -> Self::CTX {
        ProxyContext {
            config: self.config.clone(),
        }
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        let config = ctx.config.load();
        let host = session
            .get_header(http::header::HOST)
            .and_then(|head| head.to_str().ok())
            .and_then(|s| Some(s.split(":").next().unwrap_or(s)))
            .ok_or(pingora::Error::create(
                HTTPStatus(StatusCode::NOT_FOUND.into()),
                ErrorSource::Upstream,
                None,
                None,
            ))?;
        info!("host: {:?}", host);
        let service = config.servers.get(host).ok_or(pingora::Error::create(
            ErrorType::CustomCode("No host found", StatusCode::BAD_REQUEST.into()),
            ErrorSource::Upstream,
            None,
            None,
        ))?;

        let server_host = service.choose().ok_or(pingora::Error::create(
            HTTPStatus(StatusCode::NOT_FOUND.into()),
            ErrorSource::Upstream,
            None,
            None,
        ))?;

        let peer = HttpPeer::new(server_host.to_owned(), false, host.to_owned());
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
