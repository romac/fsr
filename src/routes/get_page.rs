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

#[get("/<page_slug>")]
pub fn get_page(db: State<Db>, page_slug: String) -> Template {
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
