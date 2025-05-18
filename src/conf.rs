use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// The definition of the config file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub bangs: BangsConfig,

    #[serde(default)]
    pub jumps: JumpsConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BangsConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default = "default_true")]
    pub duckduckgo: bool,

    #[serde(default)]
    pub custom: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JumpsConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default)]
    pub urls: HashMap<String, String>,
}

fn default_true() -> bool {
    true
}

impl Config {
    pub fn from_str(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }
}
