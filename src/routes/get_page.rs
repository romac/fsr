use tide::{Error, Request, Response};
use tide_tera::prelude::*;

use crate::State;

pub async fn get_page(req: Request<State>) -> Result<Response, Error> {
    let state = req.state();

    let data = state.db.as_ref().read(|data| data.clone()).await;

    let page_slug = req.param("page")?;
    let page = data.find_page(page_slug).cloned();

    if let Some(page) = page {
        state.tera.render_response(
            "page.html",
            &context! {
                "pages" => data.pages,
                "page" => page,
            },
        )
    } else {
        crate::routes::not_found(req).await
    }
}
