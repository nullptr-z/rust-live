use std::collections::HashMap;

use anyhow::{Result, anyhow};
use rand::seq::IndexedRandom;

use crate::conf::raw::*;

#[derive(Debug, Clone)]
pub struct ProxyConfigResolved {
    pub global: GlobalConfigResolved,
    pub servers: HashMap<String, ServerConfigResolved>,
}

#[derive(Debug, Clone)]
pub struct GlobalConfigResolved {
    pub port: u16,
    pub certs: Option<CertConfigResolved>,
    // pub tls: Option<TLSConfigResolved>,
}

#[derive(Debug, Clone)]
pub struct ServerConfigResolved {
    pub upstream: UpstreamConfigResolved,
    pub certs: Option<CertConfigResolved>,
    // pub tls: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CertConfigResolved {
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Debug, Clone)]
pub struct TLSConfigResolved {
    pub certs: String,
    pub key: String,
    pub ca: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpstreamConfigResolved {
    pub servers: Vec<String>,
}

impl GlobalConfigResolved {
    fn new_certs(&mut self, cert: CertConfigResolved) -> Result<&mut Self, ()> {
        self.certs = Some(cert);

        Ok(self)
    }
}

impl TryFrom<SimpleProxyConfig> for ProxyConfigResolved {
    type Error = anyhow::Error;

    fn try_from(mut value: SimpleProxyConfig) -> Result<Self> {
        let mut serversMap: HashMap<String, ServerConfigResolved> = HashMap::new();

        let mut certMap: HashMap<String, CertConfigResolved> = HashMap::new();
        for cert in value.certs {
            certMap.insert(cert.name.clone(), cert.try_into()?);
        }

        let mut upstreamMap: HashMap<String, UpstreamConfigResolved> = HashMap::new();
        for upstream in value.upstreams {
            upstreamMap.insert(upstream.name.clone(), upstream.try_into()?);
        }

        for sv in value.servers.into_iter() {
            serversMap.insert(
                sv.upstream.clone(),
                ServerConfigResolved::try_from_with_maps(&sv, &certMap, &upstreamMap)?,
            );
        }

        let tls = value.global.tls.take();
        let mut that = Self {
            global: value.global.try_into()?,
            servers: serversMap,
        };

        if let Some(certName) = tls {
            if let Some(cert) = certMap.get(&certName) {
                if let Err(_) = that.global.new_certs(cert.to_owned()) {
                    return Err(anyhow!("Failed to set global certs: "));
                }
            }
        }

        Ok(that)
    }
}

impl TryFrom<GlobalConfig> for GlobalConfigResolved {
    type Error = anyhow::Error;

    fn try_from(value: GlobalConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            port: value.port,
            certs: None,
            // tls: value.tls.map(|tls| tls.try_into()).transpose()?,
        })
    }
}

impl ServerConfigResolved {
    fn try_from_with_maps(
        server: &ServerConfig,
        cert_map: &HashMap<String, CertConfigResolved>,
        upstream_map: &HashMap<String, UpstreamConfigResolved>,
    ) -> Result<Self> {
        // Resolve TLS for this server if configured
        let certs = match &server.tls {
            Some(cert_name) => {
                let cert = cert_map
                    .get(cert_name)
                    .ok_or_else(|| anyhow!("Server TLS certificate '{}' not found", cert_name))?;
                Some(cert.clone())
            }
            None => None,
        };

        // Get the upstream configuration
        let upstream_name = &server.upstream;
        let upstream = upstream_map
            .get(upstream_name)
            .ok_or_else(|| anyhow!("Upstream '{}' not found", upstream_name))?
            .clone();

        Ok(ServerConfigResolved { certs, upstream })
    }

    pub fn choose(&self) -> Option<&str> {
        // 随机指定一个服务
        let upstream = self.upstream.servers.choose(&mut rand::rng());
        upstream.map(|m| m.as_str())
    }
}

impl TryFrom<CertConfig> for CertConfigResolved {
    type Error = anyhow::Error;

    fn try_from(value: CertConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            cert_path: value.cert_path,
            key_path: value.key_path,
        })
    }
}

impl TryFrom<TLSConfig> for TLSConfigResolved {
    type Error = ();

    fn try_from(value: TLSConfig) -> Result<Self, Self::Error> {
        if let Some(c) = value.ca {
            return Ok(Self {
                ca: Some(c.to_string_lossy().to_string()),
                certs: value.certs.to_string_lossy().to_string(),
                key: value.key.to_string_lossy().to_string(),
            });
        }
        Err(())
    }
}

impl TryFrom<UpstreamConfig> for UpstreamConfigResolved {
    type Error = anyhow::Error;

    fn try_from(value: UpstreamConfig) -> Result<Self> {
        Ok(Self {
            servers: value.servers.to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::conf::{raw::*, *};

    #[test]
    fn proxyConfigResolved_from_by_raw_should_ok() {
        let yaml: &'static str = include_str!("../../fixtures/simple.yaml");
        let config = SimpleProxyConfig::from_yaml_str(yaml).unwrap();

        let proxyConfig = ProxyConfigResolved::try_from(config);
        assert!(proxyConfig.is_ok());
        println!("【 proxyConfig 】==> {:#?}", proxyConfig);
    }
}
