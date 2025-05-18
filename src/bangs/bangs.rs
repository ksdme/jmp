use std::collections::HashMap;

use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Bang {
    #[serde(rename = "t")]
    shorthand: String,

    #[serde(rename = "u")]
    destination: String,

    #[serde(rename = "s")]
    service: String,
}

// Load all the duckduckgo bangs from the stored file.
static DUCKDUCKGO_BANGS: Lazy<Vec<Bang>> = Lazy::new(|| {
    let bangs = include_str!("duckduckgo.json");

    serde_json::from_str(&bangs).expect("bangs list should be parseable")
});

// Returns a list of custom bangs.
fn custom_bangs(bangs: &HashMap<String, String>) -> Vec<Bang> {
    bangs
        .iter()
        .map(|(shorthand, destination)| Bang {
            service: "custom".to_string(),
            shorthand: shorthand.to_string(),
            destination: destination.to_string(),
        })
        .collect()
}

fn interpolate_bang(destination: &str, query: &str) -> String {
    destination.replace("{{{s}}}", query)
}

// Returns the interpolated destination of a bang.
pub fn resolve_bang(config: crate::conf::Config, shorthand: &str, query: &str) -> Option<String> {
    let shorthand = shorthand.to_lowercase();

    // Figure out the exact match first.
    for bang in custom_bangs(&config.bangs.custom).iter() {
        if bang.shorthand.to_lowercase() == shorthand {
            return Some(interpolate_bang(&bang.destination, query));
        }
    }

    if config.bangs.duckduckgo {
        for bang in DUCKDUCKGO_BANGS.iter() {
            if bang.shorthand.to_lowercase() == shorthand {
                return Some(interpolate_bang(&bang.destination, query));
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conf::{Bangs, Config};

    fn create_test_config(
        custom_bangs: HashMap<String, String>,
        duckduckgo_enabled: bool,
    ) -> Config {
        Config {
            bangs: Bangs {
                enabled: true,
                duckduckgo: duckduckgo_enabled,
                custom: custom_bangs,
            },
            jumps: Default::default(),
        }
    }

    #[test]
    fn test_custom_bang_resolution() {
        let mut custom = HashMap::new();
        custom.insert(
            "gh".to_string(),
            "https://github.com/search?q={{{s}}}".to_string(),
        );

        let config = create_test_config(custom, false);

        assert_eq!(
            resolve_bang(config, "gh", "rust-lang/rust"),
            Some("https://github.com/search?q=rust-lang/rust".to_string())
        );
    }

    #[test]
    fn test_custom_bang_case_insensitive() {
        let mut custom = HashMap::new();
        custom.insert(
            "YT".to_string(),
            "https://youtube.com/results?search_query={{{s}}}".to_string(),
        );

        let config = create_test_config(custom, false);

        assert_eq!(
            resolve_bang(config, "yt", "rust programming"),
            Some("https://youtube.com/results?search_query=rust programming".to_string())
        );
    }

    #[test]
    fn test_duckduckgo_bang_disabled() {
        let config = create_test_config(HashMap::new(), false);

        // This should return None even if the bang exists in DuckDuckGo's list
        assert_eq!(resolve_bang(config, "w", "rust"), None);
    }

    #[test]
    fn test_nonexistent_bang() {
        let config = create_test_config(HashMap::new(), true);

        assert_eq!(resolve_bang(config, "nonexistent", "query"), None);
    }

    #[test]
    fn test_duckduckgo_bang_enabled() {
        let config = create_test_config(HashMap::new(), true);

        // Test with a known DuckDuckGo bang (rust - Rust stdlib docs)
        let result = resolve_bang(config, "rust", "Vec");
        assert!(
            result.is_some(),
            "DuckDuckGo bang 'rust' should resolve when enabled"
        );
        assert!(
            result.unwrap().contains("doc.rust-lang.org"),
            "DuckDuckGo rust bang should point to rust-lang.org"
        );
    }
}
