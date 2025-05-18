// Resolves a jump using the config.
// TODO: Sort for the longest match first.
pub fn resolve_jump(config: &crate::conf::Config, query: &str) -> Option<String> {
    let q_parts: Vec<&str> = query.split_whitespace().collect();

    // Find matching shorthand.
    for (shorthand, destination) in config.jumps.urls.iter() {
        let s_normalized = shorthand.trim();
        let s_parts: Vec<&str> = s_normalized.split_whitespace().collect();

        // Check if query starts with the shorthand
        if q_parts.len() >= s_parts.len() {
            let matches = q_parts
                .iter()
                .take(s_parts.len())
                .zip(s_parts.iter())
                .all(|(a, b)| a.to_lowercase() == *b);

            if matches {
                // Get remaining arguments after the shorthand match
                let remaining_args: Vec<&str> =
                    q_parts.iter().skip(s_parts.len()).copied().collect();

                // Inflate the destination template
                let mut inflated = destination.clone();
                for (i, arg) in remaining_args.iter().enumerate() {
                    let template = format!("{{{{{{{}}}}}}}", i + 1);
                    inflated = inflated.replace(&template, arg);
                }

                return Some(inflated);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conf::{Config, Jumps};
    use std::collections::HashMap;

    fn create_test_config(urls: HashMap<String, String>) -> Config {
        Config {
            fallback_search: Default::default(),
            bangs: Default::default(),
            jumps: Jumps {
                enabled: true,
                prefix: "go".to_string(),
                urls,
            },
        }
    }

    #[test]
    fn test_resolve_jump() {
        let mut urls = HashMap::new();
        urls.insert(
            "gh repo".to_string(),
            "https://github.com/{{{1}}}".to_string(),
        );
        urls.insert(
            "google".to_string(),
            "https://google.com/search?q={{{1}}}".to_string(),
        );

        let config = create_test_config(urls);

        // Test full shorthand match with template
        assert_eq!(
            resolve_jump(&config, "gh repo rust-lang/rust"),
            Some("https://github.com/rust-lang/rust".to_string())
        );

        // Test case insensitive match
        assert_eq!(
            resolve_jump(&config, "GH REPO example"),
            Some("https://github.com/example".to_string())
        );

        // Test single word shorthand
        assert_eq!(
            resolve_jump(&config, "google rust programming"),
            Some("https://google.com/search?q=rust".to_string())
        );

        // Test no match
        assert_eq!(resolve_jump(&config, "unknown command"), None);
    }
}
