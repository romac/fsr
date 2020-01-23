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

#[get("/thumbnail/category/<slug>")]
pub fn get_thumbnail(db: State<Db>, slug: String) -> Result<File, NotFound<String>> {
    let first_image = db.inner().as_ref().read(|data| {
        (data
            .find_category(&slug)
            .and_then(|c| c.images.first())
            .cloned())
    });

    first_image
        .ok_or_else(|| NotFound(format!("Unknown category: {}", slug)))
        .and_then(|image| {
            let path = format!("content/images/{}", image.path());
            File::open(&path).map_err(|e| NotFound(format!("Unknown image: {} ({})", path, e)))
        })
}
