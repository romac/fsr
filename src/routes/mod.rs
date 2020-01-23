mod get_page;
mod get_theme;
mod index;

pub use get_page::*;
pub use get_theme::*;
pub use index::*;

use rocket::routes;

pub fn all() -> Vec<rocket::Route> {
    routes![index, get_theme, get_page]
}
