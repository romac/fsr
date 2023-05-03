use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use axum_template::RenderHtml;
use serde::Serialize;

use crate::{data::Page, AppState};

pub async fn get_page(State(state): State<AppState>, Path(page_slug): Path<String>) -> Response {
    let data = state.db.as_ref().read(|data| data.clone()).await;

    let page = data.find_page(&page_slug).cloned();

    #[derive(Serialize)]
    struct Data {
        pages: Vec<Page>,
        page: Option<Page>,
    }

    if let Some(page) = page {
        RenderHtml(
            "page.html",
            state.engine,
            Data {
                pages: data.pages,
                page: Some(page),
            },
        )
        .into_response()
    } else {
        crate::routes::not_found(State(state)).await.into_response()
    }
}
