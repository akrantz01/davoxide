use axum::{
    body::Body,
    http::{Request, Response},
    middleware,
    routing::any,
    Extension, Router, Server,
};
use dav_server::{body::Body as DavBody, localfs::LocalFs, memls::MemLs, DavHandler};

mod authentication;

#[tokio::main]
async fn main() {
    let webdav = DavHandler::builder()
        .strip_prefix("/dav")
        .filesystem(LocalFs::new("./files", false, false, false))
        .locksystem(MemLs::new())
        .build_handler();

    let app = Router::new()
        .route("/dav/*path", any(webdav_handler))
        .layer(Extension(webdav))
        .layer(middleware::from_fn(authentication::middleware));

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
