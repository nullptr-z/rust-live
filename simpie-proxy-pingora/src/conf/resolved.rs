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
    // pub certs: Option<CertConfigResolved>,
    pub tls: Option<TLSConfigResolved>,
}

#[derive(Debug, Clone)]
pub struct ServerConfigResolved {
    pub upstream: UpstreamConfigResolved,
    pub tls: Option<TLSConfigResolved>,
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

impl TryFrom<SimpleProxyConfig> for ProxyConfigResolved {
    type Error = ();

    fn try_from(value: SimpleProxyConfig) -> Result<Self, Self::Error> {
        let mut servers: HashMap<String, ServerConfigResolved> = HashMap::new();
        for sv in value.servers.into_iter() {
            servers.insert(sv.upstream.clone(), sv.try_into()?);
        }

        Ok(Self {
            global: value.global.try_into()?,
            servers,
        })
    }
}

impl TryFrom<GlobalConfig> for GlobalConfigResolved {
    type Error = ();

    fn try_from(value: GlobalConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            port: value.port,
            tls: value.tls.map(|tls| tls.try_into()).transpose()?,
        })
    }
}

impl TryFrom<ServerConfig> for ServerConfigResolved {
    type Error = ();

    fn try_from(value: ServerConfig) -> Result<Self, Self::Error> {
        // let open_tls = value.tls.unwrap_or(false);
        Ok(Self {
            tls: value.tls.map(|op| op.try_into()).transpose()?,
            upstream: UpstreamConfigResolved {
                servers: value.server_name,
            },
        })
    }
}

impl TryFrom<CertConfig> for CertConfigResolved {
    type Error = ();

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
    type Error = ();

    fn try_from(value: UpstreamConfig) -> Result<Self, Self::Error> {
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
