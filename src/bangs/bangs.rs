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
