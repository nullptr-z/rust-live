use std::collections::HashMap;

use crate::conf::raw::*;

#[derive(Debug, Clone)]
pub struct ProxyConfigResolved {
    pub global: GlobalConfigResolved,
    pub servers: HashMap<String, ServerConfigResolved>,
}

#[derive(Debug, Clone)]
pub struct GlobalConfigResolved {
    pub port: u16,
    pub tls: Option<TlsConfigResolved>,
}

#[derive(Debug, Clone)]
pub struct ServerConfigResolved {
    pub tls: bool,
    pub upstream: UpstreamConfigResolved,
}

#[derive(Debug, Clone)]
pub struct TlsConfigResolved {
    pub cert: String,
    pub key: String,
    pub ca: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpstreamConfigResolved {
    pub servers: Vec<String>,
}

impl TryFrom<SimpleProxyConfig> for ProxyConfigResolved {
    type Error = ();

    fn try_from(value: SimpleProxyConfig) -> Result<Self, Self::Error> {
        todo!()
    }
}
