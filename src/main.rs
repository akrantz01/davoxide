use axum::{
    handler::Handler,
    middleware,
    routing::{any, post},
    Extension, Router, Server,
};
use eyre::WrapErr;
use tokio::signal;
use tracing::{info, warn};

mod config;
mod database;
mod error;
mod frontend;
mod graphql;
mod logging;
mod security;
mod webdav;

use security::{BasicAuth, SSOAuth};

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
    // The webdav and frontend routers are kept separate due to their separate authentication requirements
    let dav_router = Router::new()
        .route("/dav/*path", any(webdav::handler))
        .layer(Extension(webdav::filesystem(&config.path)))
        .layer(middleware::from_fn(security::ensure_authenticated))
        .layer(middleware::from_fn(security::extract::<_, BasicAuth>))
        .layer(middleware::from_fn(security::extract::<_, SSOAuth>));
    let frontend_router = Router::new()
        .route("/api/graphql", post(graphql::handler))
        .fallback(frontend::fallback.into_service())
        .layer(Extension(graphql::schema(config.clone(), db.clone())))
        .layer(middleware::from_fn(security::ensure_authenticated))
        .layer(middleware::from_fn(security::extract::<_, SSOAuth>));
    let app = Router::new()
        .merge(dav_router)
        .merge(frontend_router)
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
