use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The main configuration struct for Simple Proxy
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleProxyConfig {
    /// Global configuration settings
    pub global: GlobalConfig,

    /// Certificate configurations
    pub certs: Vec<CertConfig>,

    /// Server configurations
    pub servers: Vec<ServerConfig>,

    /// Upstream server configurations
    pub upstreams: Vec<UpstreamConfig>,
}

/// Global configuration settings
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalConfig {
    /// Port on which the proxy listens
    pub port: u16,

    /// TLS configuration (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TLSConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CertConfig {
    pub name: String,
    pub cert_path: String,
    pub key_path: String,
}

/// Server configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    /// List of server names (domains) this server handles
    pub server_name: Vec<String>,

    /// Name of the upstream server group to forward requests to
    pub upstream: String,

    /// Whether TLS is enabled for this server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls: Option<TLSConfig>,
}

/// TLS configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TLSConfig {
    /// Path to CA certificate file (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca: Option<PathBuf>,

    /// Path to certificate file
    pub certs: PathBuf,

    /// Path to key file
    pub key: PathBuf,
}

/// Upstream server configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpstreamConfig {
    /// Name of the upstream server group
    pub name: String,

    /// List of server addresses in this group
    pub servers: Vec<String>,
}

impl SimpleProxyConfig {
    /// Load configuration from a YAML file
    pub fn from_yaml_file(path: impl AsRef<Path>) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        let config = serde_yaml::from_reader(file)?;
        Ok(config)
    }

    /// Load configuration from YAML string
    pub fn from_yaml_str(yaml: &str) -> Result<Self> {
        let config = serde_yaml::from_str(yaml)?;
        Ok(config)
    }

    /// Save configuration to a YAML file
    pub fn to_yaml_file(&self, path: impl AsRef<Path>) -> Result<()> {
        let file = std::fs::File::create(path)?;
        serde_yaml::to_writer(file, self)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_sample_config() {
        let yaml: &'static str = include_str!("../../fixtures/simple.yaml");
        let config = SimpleProxyConfig::from_yaml_str(yaml).unwrap();
        println!("【 config 】==> {:#?}", config);

        assert_eq!(config.global.port, 8080);
        assert!(config.global.tls.is_none());

        assert_eq!(config.servers.len(), 2);
        assert_eq!(
            config.servers[0].server_name,
            vec!["acme.com", "www.acme.com"]
        );
        assert_eq!(config.servers[0].upstream, "web_servers");

        // assert_eq!(config.upstreams.len(), 2);
        // assert_eq!(config.upstreams[0].name, "web_servers");
        // assert_eq!(
        //     config.upstreams[0].servers,
        //     vec!["127.0.0.1:3001", "127.0.0.1:3002"]
        // );
    }
}
