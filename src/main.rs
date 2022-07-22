use axum::{
    body::Body,
    http::{Request, Response},
    routing::any,
    Extension, Router, Server,
};
use dav_server::{body::Body as DavBody, localfs::LocalFs, memls::MemLs, DavHandler};

#[tokio::main]
async fn main() {
    let webdav = DavHandler::builder()
        .strip_prefix("/dav")
        .filesystem(LocalFs::new("./files", false, false, false))
        .locksystem(MemLs::new())
        .build_handler();

    let app = Router::new()
        .route("/dav/*path", any(webdav_handler))
        .layer(Extension(webdav));

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
