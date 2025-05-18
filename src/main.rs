mod conf;

fn main() {
    // Example usage
    let example_config = r#"
[bangs.custom]
g = "https://google.com/search?q={}"
gh = "https://github.com/search?q={}"

[jumps]
enabled = true

[jumps.urls]
github = "https://github.com"
rust = "https://rust-lang.org"
"#;

    match conf::Config::from_str(example_config) {
        Ok(config) => {
            println!("Parsed config: {:#?}", config);
        }
        Err(e) => {
            eprintln!("Failed to parse config: {}", e);
        }
    }
}
