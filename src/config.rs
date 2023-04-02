use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr;
use std::{fs, str};
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
        let str = str::from_utf8(&buf)?;

        Ok(Self::from_str(str)?)
    }
}

impl FromStr for Config {
    type Err = de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}
