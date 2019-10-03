#![recursion_limit = "256"]
#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

#[macro_use]
extern crate serde_derive;

mod data;
mod db;
mod load;

use std::thread;
use std::time::Duration;

use lazy_static::lazy_static;
use serde::Serialize;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::response::content::Html;
use rocket::response::Responder;
use rocket::{get, routes, Rocket};
use rocket::{Request, Response, State};
use rocket_contrib::serve::{Options, StaticFiles};
use rocket_contrib::templates::Template;

use crate::db::Database;
use crate::fairings::Db;

lazy_static! {
    static ref DB: Database = Database::new("_content");
}

#[get("/")]
fn index(db: State<Db>) -> Template {
    let data = db.inner().as_ref().read(|data| data.clone());

    Template::render("index", data)
}

#[get("/<page_slug>")]
fn get_page(db: State<Db>, page_slug: String) -> Template {
    let page = db
        .inner()
        .as_ref()
        .read(|data| data.find_page(&page_slug).cloned());

    match page {
        Some(page) => Template::render("page", page),
        None => unimplemented!(),
    }
}

fn main() -> std::io::Result<()> {
    let routes = routes![index, get_page];

    rocket::ignite()
        .attach(Template::fairing())
        .attach(Db)
        .manage(Db)
        .mount("/static", StaticFiles::new("_static", Options::None))
        .mount("/", routes)
        .launch();

    Ok(())
}

mod fairings {
    use super::*;

    #[derive(Clone, Copy)]
    pub struct Db;

    impl Db {
        pub fn watch(&self) {
            thread::spawn(move || loop {
                DB.refresh();

                thread::sleep(Duration::from_millis(1000));
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
