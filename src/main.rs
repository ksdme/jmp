mod bangs;
mod conf;

fn main() {
    // Example usage
    let example_config = r#"
[bangs.custom]
gh = "https://github.com/search?q={}"

[jumps]
enabled = true

[jumps.urls]
github = "https://github.com"
rust = "https://rust-lang.org"
"#;

    let config = conf::Config::from_str(example_config).unwrap();

    let bang = bangs::resolve_bang(config, "g", "rust");
    println!("Location: {:#?}", bang);
}
