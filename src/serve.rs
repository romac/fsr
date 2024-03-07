use std::path::Path;

use axum::{
    body::Body,
    extract::Path as PathParam,
    http::{header::CONTENT_TYPE, StatusCode},
    response::Response,
};
use path_absolutize::Absolutize;

pub async fn serve_file(dir: &str, PathParam(file): PathParam<String>) -> Response {
    let file = Path::new(&file);

    let Ok(path) = file.absolutize_virtually(dir) else {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("Invalid file path"))
            .unwrap();
    };

    let mime_type = mime_guess::from_path(&path).first_or_text_plain();

    match tokio::fs::read(&path).await {
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!("File not found: {}", file.display())))
            .unwrap(),

        Ok(data) => Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, mime_type.as_ref())
            .body(Body::from(data))
            .unwrap(),
    }
}
