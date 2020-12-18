use serde_derive::Serialize;
use tide::{Error, Request, Response};
use tide_tera::prelude::*;

use crate::{
    data::{Category, Page},
    db::Database,
    Db, State,
};

pub async fn get_theme(req: Request<State>) -> tide::Result<tide::Response> {
    let state = req.state();
    let theme_slug = req.param("theme")?;

    let (data, category) = state
        .db
        .as_ref()
        .read(|data| (data.clone(), data.find_category(theme_slug).cloned()))
        .await;

    if let Some(category) = category {
        state.tera.render_response(
            "theme.html",
            &context! {
                "pages" => data.pages,
                "category" => category,
            },
        )
    } else {
        crate::routes::not_found(req).await
    }
}
