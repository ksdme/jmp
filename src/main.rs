mod bangs;
mod conf;
mod jumps;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Example usage
    let config_str = r#"
[bangs.custom]
gh = "https://github.com/search?q={{{s}}}"

[jumps]
enabled = true

[jumps.urls]
github = "https://github.com"
rust = "https://rust-lang.org/{{{1}}}/{{{2}}}"
"#;
    let config = conf::Config::from_str(config_str).unwrap();

    // Set up the handler.
    let app = axum::Router::new()
        .route("/", axum::routing::get(jmp))
        .with_state(config)
        .layer(TraceLayer::new_for_http());

    // Run the app
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("could not bind to port");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct SearchParams {
    q: Option<String>,
}

// Check for bangs or jumps.
async fn jmp(State(config): State<conf::Config>, Query(params): Query<SearchParams>) -> Response {
    let q = params.q.unwrap_or_default();
    let q = q.trim();
    if q.is_empty() {
        return "nowhere to jmp".into_response();
    }

    // Resolve bangs.
    if config.bangs.enabled && q.starts_with("!") {
        if let Some(url) = bangs::resolve_bang(&config, &q[1..]) {
            return Redirect::temporary(&url).into_response();
        }
    }

    // Resolve jumps.
    let prefix = format!("{}/", config.jumps.prefix);
    if config.jumps.enabled && q.to_lowercase().starts_with(&prefix) {
        if let Some(url) = jumps::resolve_jump(&config, &q[prefix.len()..]) {
            return Redirect::temporary(&url).into_response();
        }
    }

    // Fallback to the search engine.
    let url = config.fallback_search.replace("{{{s}}}", q);
    Redirect::temporary(&url).into_response()
}
