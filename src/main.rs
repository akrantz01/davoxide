use axum::{
    middleware,
    routing::{any, post},
    Extension, Router, Server,
};
use eyre::WrapErr;
use std::env;
use tokio::signal;
use tracing::{info, warn};

mod authentication;
mod database;
mod error;
mod graphql;
mod logging;
mod webdav;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    if dotenv::dotenv().is_err() {
        warn!(".env file not found");
    }
    tracing_subscriber::fmt::init();

    let db = {
        let url = env::var("DATABASE_URL").wrap_err("missing DATABASE_URL in environment")?;
        database::connect(url).await?
    };

    // Configure routes
    let app = Router::new()
        .route("/dav/*path", any(webdav::handler))
        .route("/graphql", post(graphql::handler))
        .layer(Extension(webdav::filesystem()))
        .layer(Extension(graphql::schema(db.clone())))
        .layer(middleware::from_fn(authentication::middleware))
        .layer(Extension(db))
        .layer(logging::layer());

    // Setup shutdown handler for Ctrl+C and SIGTERM
    let shutdown = || async {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install ctrl+c handler")
        };
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install terminate signal handler")
                .recv()
                .await
        };

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }

        info!("server successfully shutdown");
        info!("goodbye :)");
    };

    let address = env::var("ADDRESS")
        .unwrap_or_else(|_| String::from("127.0.0.1:3000"))
        .parse()
        .wrap_err("invalid address format")?;

    // Launch the server
    info!(%address, "listening and ready to handle requests");
    Server::bind(&address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown())
        .await
        .wrap_err("failed to start server")?;

    Ok(())
}
