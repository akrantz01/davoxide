use axum::{
    middleware,
    routing::{any, post},
    Extension, Router, Server,
};
use eyre::WrapErr;
use tokio::signal;
use tracing::{info, warn};

mod authentication;
mod config;
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

    let config = config::load().wrap_err("failed to load config")?;

    let db = database::connect(&config.database_url).await?;

    // Configure routes
    let app = Router::new()
        .route("/dav/*path", any(webdav::handler))
        .route("/graphql", post(graphql::handler))
        .layer(Extension(webdav::filesystem(&config.path)))
        .layer(Extension(graphql::schema(config.clone(), db.clone())))
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

    // Launch the server
    info!(address = %config.address, "listening and ready to handle requests");
    Server::bind(&config.address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown())
        .await
        .wrap_err("failed to start server")?;

    Ok(())
}
