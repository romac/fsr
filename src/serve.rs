use std::path::PathBuf;

use axum::{
    body::{self, Empty, Full},
    extract::Path,
    http::{header, HeaderValue, StatusCode},
    response::Response,
};

pub async fn serve_file(dir: &str, Path(file): Path<String>) -> Response {
    let path = PathBuf::from(dir).join(file);

    let mime_type = mime_guess::from_path(&path).first_or_text_plain();

    match tokio::fs::read(&path).await {
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Ok(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file)))
            .unwrap(),
    }
}
