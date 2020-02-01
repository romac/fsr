#![recursion_limit = "256"]
#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

#[macro_use]
extern crate serde_derive;

mod data;
mod db;
mod fairings;
mod load;
mod routes;

use std::fs::File;
use std::path::Path;
use std::thread;
use std::time::Duration;

use once_cell::sync::Lazy;
use serde::Serialize;

use hotwatch::Hotwatch;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::response::content::Html;
use rocket::response::status::NotFound;
use rocket::response::Responder;
use rocket::{catch, catchers, get, routes, Rocket};
use rocket::{Request, Response, State};
use rocket_contrib::compression::Compression;
use rocket_contrib::serve::{Options, StaticFiles};
use rocket_contrib::templates::Template;

use crate::data::{Category, Page};
use crate::db::Database;
use crate::fairings::Db;

static DB_PATH: &str = "content";
static DB: Lazy<Database> = Lazy::new(|| Database::new(DB_PATH));

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let data = DB.read(|data| data.clone());

    #[derive(Clone, Serialize)]
    struct Tmpl {
        pages: Vec<Page>,
    }

    let tmpl = Tmpl { pages: data.pages };
    Template::render("not_found", tmpl)
}

fn launch() -> std::io::Result<()> {
    let routes = crate::routes::all();

    let mut watcher = Hotwatch::new().unwrap();
    watcher.watch(DB_PATH, |e| DB.refresh()).unwrap();

    DB.force_refresh();

    rocket::ignite()
        .attach(Template::fairing())
        .manage(Db)
        .attach(Compression::fairing())
        .mount("/static", StaticFiles::new("static", Options::None))
        .mount("/images", StaticFiles::new("content/images", Options::None))
        .mount("/", routes)
        .register(catchers![not_found])
        .launch();

    Ok(())
}

fn main() -> std::io::Result<()> {
    launch()
}
