use axum::{extract::State, response::IntoResponse};
use axum_template::RenderHtml;

use crate::AppState;

pub async fn get_virtual_expo(State(state): State<AppState>) -> impl IntoResponse {
    let data = state.db.as_ref().read(|data| data.clone()).await;

    RenderHtml("virtual_expo.html", state.engine, data)
}
