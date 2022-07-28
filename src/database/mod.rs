use eyre::WrapErr;
use sqlx::{
    postgres::{PgConnectOptions, PgPool},
    ConnectOptions,
};
use std::str::FromStr;
use tracing::{info, instrument, log::LevelFilter};

mod permission;
mod types;
mod user;

pub use permission::Permission;
pub use types::Action;
pub use user::User;

/// Connect to the database and run any pending migrations
#[instrument(skip_all)]
pub async fn connect(url: &str) -> eyre::Result<PgPool> {
    let options = PgConnectOptions::from_str(url)
        .wrap_err("invalid database url format")?
        .log_statements(LevelFilter::Debug)
        .to_owned();

    let db = PgPool::connect_with(options)
        .await
        .wrap_err("failed to connect to the database")?;
    info!("database connected");

    sqlx::migrate!()
        .run(&db)
        .await
        .wrap_err("failed to run database migrations")?;
    info!("database schema up-to-date");

    Ok(db)
}
