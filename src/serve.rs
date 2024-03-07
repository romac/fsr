use std::path::PathBuf;

use axum::{
    body::Body,
    extract::Path,
    http::{header::CONTENT_TYPE, StatusCode},
    response::Response,
};

pub async fn serve_file(dir: &str, Path(file): Path<String>) -> Response {
    let path = PathBuf::from(dir).join(&file);
    let mime_type = mime_guess::from_path(&path).first_or_text_plain();

    match tokio::fs::read(&path).await {
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!("File not found: {file}")))
            .unwrap(),

        Ok(file) => Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, mime_type.as_ref())
            .body(Body::from(file))
            .unwrap(),
    }
}
