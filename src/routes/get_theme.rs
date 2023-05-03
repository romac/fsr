use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use axum_template::RenderHtml;
use serde::Serialize;

use crate::{
    data::{Category, Page},
    AppState,
};

pub async fn get_theme(State(state): State<AppState>, Path(theme_slug): Path<String>) -> Response {
    let (data, category) = state
        .db
        .as_ref()
        .read(|data| (data.clone(), data.find_category(&theme_slug).cloned()))
        .await;

    #[derive(Serialize)]
    struct Data {
        pages: Vec<Page>,
        category: Option<Category>,
    }

    if let Some(category) = category {
        RenderHtml(
            "theme.html",
            state.engine,
            Data {
                pages: data.pages,
                category: Some(category),
            },
        )
        .into_response()
    } else {
        crate::routes::not_found(State(state)).await.into_response()
    }
}
