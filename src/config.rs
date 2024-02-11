use crate::error;
use ipnet::IpNet;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::net::IpAddr;
use std::path::Path;
use std::str::FromStr;
use std::{fs, str};
use toml::de;

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default)]
    pub users: BTreeMap<String, String>,
    #[serde(default)]
    pub whitelist: Vec<IpNet>,
    #[serde(default)]
    pub blacklist: Vec<IpNet>,
}

impl Config {
    pub fn from_path(path: impl AsRef<Path>) -> error::Result<Self> {
        let path = path.as_ref();

        let buf = fs::read(path)?;
        let str = str::from_utf8(&buf)?;

        Ok(Self::from_str(str)?)
    }

    pub fn is_blacklisted(&self, ip: &IpAddr) -> bool {
        self.blacklist.iter().any(|n| n.contains(ip))
    }

    pub fn is_whitelisted(&self, ip: &IpAddr) -> bool {
        self.whitelist.is_empty() || self.whitelist.iter().any(|n| n.contains(ip))
    }
}

impl FromStr for Config {
    type Err = de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}
