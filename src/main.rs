#![recursion_limit = "256"]
#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

mod data;
mod db;
mod load;
mod routes;

use std::fs::File;
use std::path::Path;
use std::thread;
use std::time::Duration;

use async_std::prelude::FutureExt;
use async_std::task;

use once_cell::sync::Lazy;
use serde::Serialize;

use tera::Tera;
use tide_compress::CompressMiddleware;

use crate::data::{Category, Page};
use crate::db::Database;

static DB_PATH: &str = "content";
static TEMPLATES_PATH: &str = "template";

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
    app.at("/expo-virtuelle").get(routes::get_virtual_expo);
    app.at("/:page").get(routes::get_page);

    app.at("/static").serve_dir("static")?;
    app.at("/images").serve_dir("content/images")?;

    app.at("*").all(routes::not_found);

    app.listen("127.0.0.1:8081").await?;

    Ok(())
}

fn watch() -> impl notify::Watcher {
    use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};

    let mut watcher: RecommendedWatcher = Watcher::new_immediate(|res| match res {
        Ok(_) => {
            task::block_on(DB.refresh());
        }
        Err(e) => {
            println!("watch error: {:?}", e);
        }
    })
    .unwrap();

    watcher.watch(DB_PATH, RecursiveMode::Recursive).unwrap();

    watcher
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    DB.force_refresh().await;
    let _watcher = watch();

    launch().await
}
