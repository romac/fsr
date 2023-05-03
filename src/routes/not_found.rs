use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_template::RenderHtml;
use serde::Serialize;

use crate::{data::Page, AppState};

#[derive(Clone, Serialize)]
struct Tmpl {
    pages: Vec<Page>,
}

pub async fn not_found(State(state): State<AppState>) -> Response {
    let data = state.db.as_ref().read(|data| data.clone()).await;

    let mut res =
        RenderHtml("not_found.html", state.engine, Tmpl { pages: data.pages }).into_response();

    *res.status_mut() = StatusCode::NOT_FOUND;
    res
}
