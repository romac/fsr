mod get_image;
mod get_page;
mod get_theme;
mod get_thumbnail;
mod index;

pub use get_image::*;
pub use get_page::*;
pub use get_theme::*;
pub use get_thumbnail::*;
pub use index::*;

use rocket::routes;

pub fn all() -> Vec<rocket::Route> {
    routes![index, get_theme, get_thumbnail, get_page]
}
