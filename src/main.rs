#![recursion_limit = "256"]
#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

#[macro_use]
extern crate serde_derive;

mod data;
mod db;
mod load;

use std::fs::File;
use std::thread;
use std::time::Duration;

use once_cell::sync::Lazy;
use serde::Serialize;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::response::content::Html;
use rocket::response::status::NotFound;
use rocket::response::Responder;
use rocket::{get, routes, Rocket};
use rocket::{Request, Response, State};
use rocket_contrib::serve::{Options, StaticFiles};
use rocket_contrib::templates::Template;

use crate::data::{Category, Page};
use crate::db::Database;
use crate::fairings::Db;

static DB: Lazy<Database> = Lazy::new(|| Database::new("content"));

#[get("/")]
fn index(db: State<Db>) -> Template {
    let data = db.inner().as_ref().read(|data| data.clone());
    Template::render("index", data)
}

#[get("/<page_slug>")]
fn get_page(db: State<Db>, page_slug: String) -> Template {
    let data = db.inner().as_ref().read(|data| data.clone());

    let page = db
        .inner()
        .as_ref()
        .read(|data| data.find_page(&page_slug).cloned());

    #[derive(Clone, Serialize)]
    struct Tmpl {
        pages: Vec<Page>,
        page: Page,
    }

    match page {
        Some(page) => {
            let tmpl = Tmpl {
                pages: data.pages,
                page,
            };
            Template::render("page", tmpl)
        }
        None => unimplemented!(),
    }
}

#[get("/theme/<theme_slug>")]
fn get_theme(db: State<Db>, theme_slug: String) -> Template {
    let (data, category) = db
        .inner()
        .as_ref()
        .read(|data| (data.clone(), data.find_category(&theme_slug).cloned()));

    #[derive(Clone, Serialize)]
    struct Tmpl {
        pages: Vec<Page>,
        category: Category,
    }

    match category {
        Some(category) => {
            let data = Tmpl {
                pages: data.pages,
                category,
            };
            Template::render("theme", data)
        }
        None => todo!(),
    }
}

#[get("/image/<slug>")]
fn get_image(db: State<Db>, slug: String) -> Result<File, NotFound<String>> {
    let path = format!("images/{}.jpg", slug);
    File::open(&path).map_err(|e| NotFound(format!("Unknown image: {}", slug)))
}

#[get("/thumbnail/category/<slug>")]
fn get_category_thumbnail(db: State<Db>, slug: String) -> Result<File, NotFound<String>> {
    let first_image = db.inner().as_ref().read(|data| {
        (data
            .find_category(&slug)
            .and_then(|c| c.images.first())
            .cloned())
    });

    first_image
        .ok_or_else(|| NotFound(format!("Unknown image: {}", slug)))
        .and_then(|image| {
            let path = format!("/images/{}.jpg", image.id);
            File::open(&path).map_err(|e| NotFound(format!("Unknown image: {}", slug)))
        })
}

fn launch() -> std::io::Result<()> {
    let routes = routes![index, get_theme, get_page];

    rocket::ignite()
        .attach(Template::fairing())
        .attach(Db)
        .manage(Db)
        .mount("/static", StaticFiles::new("static", Options::None))
        .mount("/", routes)
        .launch();

    Ok(())
}

fn main() -> std::io::Result<()> {
    launch()
}

mod fairings {
    use super::*;

    #[derive(Clone, Copy)]
    pub struct Db;

    impl Db {
        pub fn watch(self) {
            thread::spawn(move || loop {
                DB.refresh();

                thread::sleep(Duration::from_secs(30));
            });
        }
    }

    impl AsRef<Database> for Db {
        fn as_ref(&self) -> &Database {
            &DB
        }
    }

    impl Fairing for Db {
        fn info(&self) -> Info {
            Info {
                name: "Database",
                kind: Kind::Attach,
            }
        }

        fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
            self.watch();
            Ok(rocket)
        }
    }
}
