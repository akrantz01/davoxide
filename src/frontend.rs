use axum::{
    body::{boxed, Full},
    http::{header, Uri},
    response::{IntoResponse, Response},
};
use rust_embed::{EmbeddedFile, RustEmbed};

#[derive(RustEmbed)]
#[folder = "frontend/dist/"]
struct Assets;

/// Fallback to the index router as the frontend is a single page app
pub async fn fallback(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    let path = if path.is_empty() { "index.html" } else { path };

    // Load the asset
    match Assets::get(path) {
        Some(content) => asset_to_response(content, path),
        None => asset_to_response(Assets::get("index.html").unwrap(), "index.html"),
    }
}

// Convert an embedded asset to a response
pub fn asset_to_response(asset: EmbeddedFile, path: &str) -> Response {
    let body = boxed(Full::from(asset.data));
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    Response::builder()
        .header(header::CONTENT_TYPE, mime.as_ref())
        .body(body)
        .unwrap()
}
