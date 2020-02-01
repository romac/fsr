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

#[derive(Clone, Serialize)]
struct Tmpl {
    pages: Vec<Page>,
    category: Category,
}

#[get("/theme/<theme_slug>")]
pub fn get_theme(db: State<Db>, theme_slug: String) -> Option<Template> {
    let (data, category) = db
        .inner()
        .as_ref()
        .read(|data| (data.clone(), data.find_category(&theme_slug).cloned()));

    category.map(|category| {
        let data = Tmpl {
            pages: data.pages,
            category,
        };

        Template::render("theme", data)
    })
}
