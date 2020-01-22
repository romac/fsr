#![recursion_limit = "256"]
#![feature(proc_macro_hygiene, decl_macro)]
#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

#[macro_use]
extern crate serde_derive;

mod data;
mod db;
mod load;
mod routes;

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

fn launch() -> std::io::Result<()> {
    let routes = crate::routes::all();

    rocket::ignite()
        .attach(Template::fairing())
        .attach(Db)
        .manage(Db)
        .mount("/static", StaticFiles::new("static", Options::None))
        .mount("/images", StaticFiles::new("content/images", Options::None))
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
