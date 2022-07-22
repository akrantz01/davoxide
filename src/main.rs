use axum::{
    body::Body,
    http::{Request, Response},
    middleware,
    routing::any,
    Extension, Router, Server,
};
use dav_server::{body::Body as DavBody, localfs::LocalFs, memls::MemLs, DavHandler};
use tracing::warn;

mod authentication;
mod logging;

#[tokio::main]
async fn main() {
    if dotenv::dotenv().is_err() {
        warn!(".env file not found");
    }
    tracing_subscriber::fmt::init();

    let webdav = DavHandler::builder()
        .strip_prefix("/dav")
        .filesystem(LocalFs::new("./files", false, false, false))
        .locksystem(MemLs::new())
        .build_handler();

    let app = Router::new()
        .route("/dav/*path", any(webdav_handler))
        .layer(Extension(webdav))
        .layer(middleware::from_fn(authentication::middleware))
        .layer(logging::layer());

    Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn webdav_handler(
    Extension(webdav): Extension<DavHandler>,
    req: Request<Body>,
) -> Response<DavBody> {
    webdav.handle(req).await
}
