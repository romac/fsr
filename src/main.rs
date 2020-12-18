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

use once_cell::sync::Lazy;
use serde::Serialize;

use hotwatch::Hotwatch;
use tera::Tera;
use tide_compress::CompressMiddleware;

use crate::data::{Category, Page};
use crate::db::Database;

static DB_PATH: &str = "content";
static TEMPLATES_PATH: &str = "template";

static DB: Lazy<Database> = Lazy::new(|| Database::new(DB_PATH));

// #[catch(404)]
// fn not_found(req: &Request) -> Template {
//     let data = DB.read(|data| data.clone());

//     #[derive(Clone, Serialize)]
//     struct Tmpl {
//         pages: Vec<Page>,
//     }

//     let tmpl = Tmpl { pages: data.pages };
//     Template::render("not_found", tmpl)
// }

fn watch() {
    // let mut watcher = Hotwatch::new().unwrap();
    // watcher.watch(DB_PATH, |e| DB.refresh()).unwrap();

    async_std::task::spawn_blocking(|| loop {
        DB.force_refresh();
        thread::sleep(Duration::from_secs(10));
    });
}

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

#[async_std::main]
async fn main() -> tide::Result<()> {
    watch();

    // tide::log::start();

    let mut tera = Tera::new("templates/**/*")?;
    tera.autoescape_on(vec!["html"]);

    let state = State { db: Db, tera };

    let mut app = tide::with_state(state);
    app.with(CompressMiddleware::new());

    app.at("/").get(routes::index);
    app.at("/:page").get(routes::get_page);
    app.at("/theme/:theme").get(routes::get_theme);

    app.at("/static").serve_dir("static")?;
    app.at("/images").serve_dir("content/images")?;

    app.listen("127.0.0.1:8081").await?;

    Ok(())
}
