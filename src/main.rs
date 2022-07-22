use axum::{
    body::Body,
    http::{Request, Response},
    middleware,
    routing::any,
    Extension, Router, Server,
};
use dav_server::{body::Body as DavBody, localfs::LocalFs, memls::MemLs, DavHandler};
use eyre::WrapErr;
use std::env;
use tracing::{info, warn};

mod authentication;
mod logging;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    if dotenv::dotenv().is_err() {
        warn!(".env file not found");
    }
    tracing_subscriber::fmt::init();

    let webdav = DavHandler::builder()
        .strip_prefix("/dav")
        .filesystem(LocalFs::new("./files", false, false, false))
        .locksystem(MemLs::new())
        .build_handler();

    // Configure routes
    let app = Router::new()
        .route("/dav/*path", any(webdav_handler))
        .layer(Extension(webdav))
        .layer(middleware::from_fn(authentication::middleware))
        .layer(logging::layer());

    let address = env::var("ADDRESS")
        .unwrap_or_else(|_| String::from("127.0.0.1:3000"))
        .parse()
        .wrap_err("invalid address format")?;

    // Launch the server
    info!(%address, "listening and ready to handle requests");
    Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .wrap_err("failed to start server")?;

    Ok(())
}

async fn webdav_handler(
    Extension(webdav): Extension<DavHandler>,
    req: Request<Body>,
) -> Response<DavBody> {
    webdav.handle(req).await
}
