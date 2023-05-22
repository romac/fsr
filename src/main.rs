mod data;
mod db;
mod load;
mod routes;
mod serve;
mod watch;

use axum::{routing::get, Router};
use axum_template::engine::Engine;
use once_cell::sync::Lazy;
use tera::Tera;
use tokio::task;
use tower_http::compression::CompressionLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use crate::db::Database;
use crate::serve::serve_file;
use crate::watch::watch;

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
        .route("/expo-en-cours", get(routes::get_virtual_expo))
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

    let addr = "0.0.0.0:8081".parse().unwrap();
    info!("Listening on http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    DB.refresh().await;

    task::spawn(watch(DB_PATH));

    launch().await
}
