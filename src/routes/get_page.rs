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
    page: Page,
}

pub async fn get_page(req: Request<State>) -> Result<Response, Error> {
    let state = req.state();

    let data = state.db.as_ref().read(|data| data.clone());

    let page_slug = req.param("page")?;
    let page = data.find_page(page_slug).cloned();

    if let Some(page) = page {
        let tmpl = Tmpl {
            pages: data.pages,
            page,
        };

        state
            .tera
            .render_response("page.html", &tera::Context::from_serialize(tmpl)?)
    } else {
        Err(Error::from_str(
            tide::StatusCode::NotFound,
            "page introuvable",
        ))
    }
}
