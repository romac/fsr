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

#[get("/image/<slug>")]
pub fn get_image(db: State<Db>, slug: String) -> Result<File, NotFound<String>> {
    let path = format!("images/{}.jpg", slug);
    File::open(&path).map_err(|e| NotFound(format!("Unknown image: {}", slug)))
}
