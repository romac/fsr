mod data;
mod db;
mod load;
mod routes;

use std::path::Path;

use async_std::{
    channel::{self, Receiver},
    stream::StreamExt,
    task,
};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::Lazy;

use tera::Tera;
use tide::log::{debug, error};
use tide_compress::CompressMiddleware;

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
pub struct State {
    pub db: Db,
    pub tera: Tera,
}

async fn launch() -> tide::Result<()> {
    let mut tera = Tera::new("templates/**/*")?;
    tera.autoescape_on(vec!["html"]);

    let state = State { db: Db, tera };

    let mut app = tide::with_state(state);
    app.with(CompressMiddleware::new());

    app.at("/").get(routes::get_index);
    app.at("/theme/:theme").get(routes::get_theme);
    app.at("/expo-en-cours").get(routes::get_virtual_expo);
    app.at("/:page").get(routes::get_page);

    app.at("/static").serve_dir("static")?;
    app.at("/images").serve_dir("content/images")?;

    app.at("*").all(routes::not_found);

    app.listen("0.0.0.0:8081").await?;

    Ok(())
}

fn watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<notify::Event>>)> {
    let (tx, rx) = channel::bounded(1);

    let watcher = RecommendedWatcher::new(
        move |res| {
            task::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn watch(path: impl AsRef<Path>) -> notify::Result<()> {
    let (mut watcher, mut rx) = watcher()?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                debug!("files changed: {:?}", event.paths);
                DB.refresh().await;
            }
            Err(e) => error!("watch error: {:?}", e),
        }
    }

    Ok(())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::with_level(tide::log::LevelFilter::Info);

    DB.refresh().await;

    task::spawn_local(watch(DB_PATH));

    launch().await
}
