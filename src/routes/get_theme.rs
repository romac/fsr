use serde_derive::Serialize;
use tide::{Error, Request, Response};
use tide_tera::TideTeraExt;

use crate::{
    data::{Category, Page},
    db::Database,
    Db, State,
};

#[derive(Clone, Serialize)]
struct Tmpl {
    pages: Vec<Page>,
    category: Category,
}

pub async fn get_theme(req: Request<State>) -> tide::Result<tide::Response> {
    let state = req.state();
    let theme_slug = req.param("theme")?;

    let (data, category) = state
        .db
        .as_ref()
        .read(|data| (data.clone(), data.find_category(theme_slug).cloned()));

    if let Some(category) = category {
        let tmpl = Tmpl {
            pages: data.pages,
            category,
        };

        state
            .tera
            .render_response("theme.html", &tera::Context::from_serialize(tmpl)?)
    } else {
        Err(Error::from_str(
            tide::StatusCode::NotFound,
            "page introuvable",
        ))
    }
}
