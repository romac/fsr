use serde::Serialize;
use tide::{Error, Request, Response, StatusCode};
use tide_tera::prelude::*;

use crate::{data::Page, State};

#[derive(Clone, Serialize)]
struct Tmpl {
    pages: Vec<Page>,
}

pub async fn not_found(req: Request<State>) -> Result<Response, Error> {
    let state = req.state();
    let data = state.db.as_ref().read(|data| data.clone()).await;

    let mut res = state.tera.render_response(
        "not_found.html",
        &context! {
            "pages" => data.pages,
        },
    )?;

    res.set_status(StatusCode::NotFound);

    Ok(res)
}
