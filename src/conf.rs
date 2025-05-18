use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// The definition of the config file.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_fallback_search")]
    pub fallback_search: String,

    #[serde(default)]
    pub bangs: Bangs,

    #[serde(default)]
    pub jumps: Jumps,
}

// :)
fn default_fallback_search() -> String {
    "https://google.com/search?q={{{s}}}".to_string()
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Bangs {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default = "default_true")]
    pub duckduckgo: bool,

    #[serde(default)]
    pub custom: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Jumps {
    #[serde(default = "default_true")]
    pub enabled: bool,

    #[serde(default = "default_prefix")]
    pub prefix: String,

    #[serde(default)]
    pub urls: HashMap<String, String>,
}

fn default_true() -> bool {
    true
}

fn default_prefix() -> String {
    "go".to_string()
}

impl Config {
    pub fn from_str(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }
}
