use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use toml::de;

#[derive(Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub server: Server,
    #[serde(default)]
    pub users: BTreeMap<String, String>,
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> color_eyre::Result<Self> {
        let path = path.as_ref();
        let buf = fs::read(path)?;

        Ok(Self::from_slice(&buf)?)
    }

    fn from_slice(bytes: &[u8]) -> Result<Self, de::Error> {
        toml::from_slice(bytes)
    }
}

impl Debug for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("server", &self.server)
            .finish()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Server {
    #[serde(default = "default_socket_addr")]
    pub addr: SocketAddr,
    #[serde(default)]
    pub auth: bool,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            addr: default_socket_addr(),
            auth: Default::default(),
        }
    }
}

fn default_socket_addr() -> SocketAddr {
    SocketAddr::new(IpAddr::from([127, 0, 0, 1]), 1080)
}
