mod raw;
mod resolved;

use anyhow::Result;
use arc_swap::ArcSwap;
pub use raw::*;
pub use resolved::*;
use std::{path::Path, sync::Arc};

#[derive(Debug, Clone)]
pub struct ProxyConfig(Arc<ArcSwap<ProxyConfigResolved>>);

impl ProxyConfig {
    pub fn new(config: ProxyConfigResolved) -> Self {
        let config = Arc::new(ArcSwap::new(Arc::new(config)));
        Self(config)
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let config = SimpleProxyConfig::from_yaml_file(path)?;
        Ok(Self::new(config.try_into()?))
    }

    pub fn update(&self, config: ProxyConfigResolved) {
        self.store(Arc::new(config));
    }

    // pub fn get_full(&self) -> Arc<ProxyConfigResolved> {
    //     self.load_full()
    // }
}

impl std::ops::Deref for ProxyConfig {
    type Target = ArcSwap<ProxyConfigResolved>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
