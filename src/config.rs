use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use toml::de;

#[derive(Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
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
