mod data;
mod db;
mod load;
mod routes;

use std::path::PathBuf;

use axum::{
    body::{self, Empty, Full},
    extract::Path,
    http::{header, HeaderValue, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use axum_template::engine::Engine;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::Lazy;
use tera::Tera;
use tokio::{
    runtime::Handle,
    select,
    sync::mpsc::{channel, Receiver},
    task,
};
use tower_http::{
    compression::CompressionLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{debug, error, info, warn, Level};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

use crate::db::Database;

static DB_PATH: &str = "content";

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
    let mut tera = Tera::new("templates/**/*")?;
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
        .route("/static/*path", get(move |path| serve_file("static", path)))
        .route(
            "/images/*path",
            get(move |path| serve_file("content/images", path)),
        )
        .fallback(routes::not_found)
        .layer(CompressionLayer::new())
        .layer(trace_layer)
        .with_state(state);

    let quit_sig = async {
        _ = tokio::signal::ctrl_c().await;
        warn!("Initiating graceful shutdown");
    };

    let addr = "0.0.0.0:8081".parse().unwrap();
    info!("Listening on http://{addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(quit_sig)
        .await?;

    Ok(())
}

async fn serve_file(dir: &str, Path(file): Path<String>) -> Response {
    let path = PathBuf::from(dir).join(file);

    let mime_type = mime_guess::from_path(&path).first_or_text_plain();

    match tokio::fs::read(&path).await {
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Ok(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file)))
            .unwrap(),
    }
}

fn watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<notify::Event>>)> {
    let (tx, rx) = channel(1);

    let handle = Handle::current();

    let watcher = RecommendedWatcher::new(
        move |res| {
            handle.block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn watch(path: impl AsRef<std::path::Path>) -> notify::Result<()> {
    let (mut watcher, mut rx) = watcher()?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    loop {
        select! {
            _ = tokio::signal::ctrl_c() => return Ok(()),

            res = rx.recv() => {
                match res {
                    None => return Ok(()),
                    Some(Err(e)) => error!("watch error: {:?}", e),
                    Some(Ok(event)) => {
                        debug!("files changed: {:?}", event.paths);
                        DB.refresh().await;
                    }
                }
            }
        }
    }
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
