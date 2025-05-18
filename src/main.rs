mod bangs;
mod conf;
mod jumps;

use std::fs;

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// Path to the config file.
    #[arg(long, default_value = "jmp.toml")]
    config: String,

    /// Address to bind the server to.
    #[arg(long, default_value = "127.0.0.1:62754")]
    bind: String,

    /// Log level to use.
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(args.log_level))
        .init();

    // Set up config file.
    let config = fs::read_to_string(args.config).expect("could not read config file");
    let config = conf::Config::from_str(&config).expect("could not parse config file");

    // Set up the handler.
    let app = axum::Router::new()
        .route("/", axum::routing::get(jmp))
        .with_state(config)
        .layer(TraceLayer::new_for_http());

    // Run the app.
    tracing::info!("starting server on {}", args.bind);
    let listener = tokio::net::TcpListener::bind(args.bind)
        .await
        .expect("could not bind to address");
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
