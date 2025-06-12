#![warn(unused_extern_crates)]

mod data;
mod db;
mod load;
mod routes;
mod serve;
// mod watch;

use std::time::Duration;

use axum::{routing::get, Router};
use axum_template::engine::Engine;
use once_cell::sync::Lazy;
use tera::Tera;
use tokio::task;
use tokio::time::sleep;
use tower_http::compression::CompressionLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use crate::db::Database;
use crate::serve::serve_file;
// use crate::watch::watch;

static DB_PATH: &str = "_data/content";

static DB: Lazy<Database> = Lazy::new(|| Database::new(DB_PATH));

#[derive(Clone, Copy)]
pub struct Db;

impl AsRef<Database> for Db {
    fn as_ref(&self) -> &Database {
        &DB
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub engine: Engine<Tera>,
}

async fn launch() -> Result<()> {
    let mut tera = Tera::new("_data/templates/**/*")?;
    tera.autoescape_on(vec!["html"]);

    let state = AppState {
        db: Db,
        engine: Engine::from(tera),
    };

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new().level(Level::INFO), // .latency_unit(LatencyUnit::Micros),
        );

    let app = Router::new()
        .route("/", get(routes::get_index))
        .route("/theme/:theme", get(routes::get_theme))
        .route("/:page", get(routes::get_page))
        .route(
            "/static/*path",
            get(move |path| serve_file("_data/static", path)),
        )
        .route(
            "/images/*path",
            get(move |path| serve_file("_data/content/images", path)),
        )
        .fallback(routes::not_found)
        .layer(CompressionLayer::new())
        .layer(trace_layer)
        .with_state(state);

    let addr = "0.0.0.0:8081";
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("Listening on http://{addr}");
    axum::serve(listener, app).await?;

    Ok(())
}

fn setup_tracing() {
    use tracing_subscriber::{fmt, EnvFilter};

    fmt::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive("fsr=info".parse().unwrap())
                .from_env_lossy(),
        )
        .init();
}

fn setup_signal_handler() {
    ctrlc::set_handler(move || {
        info!("Received SIGINT, shutting down");
        std::process::exit(0);
    })
    .expect("failed to install signal handler");
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    setup_tracing();
    setup_signal_handler();

    DB.refresh().await;

    // task::spawn(watch(DB_PATH));
    task::spawn(refresh());

    launch().await
}

async fn refresh() {
    loop {
        sleep(Duration::from_secs(5)).await;
        DB.refresh().await;
    }
}
