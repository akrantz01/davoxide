use eyre::WrapErr;
use migrations::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::{info, instrument, log::LevelFilter};

mod entities;
mod permission_manager;
mod user_manager;

pub use entities::{permission::Model as Permission, user::Model as User, Action};
pub use permission_manager::PermissionManager;
pub use user_manager::UserManager;

/// Connect to the database and run any pending migrations
#[instrument(skip_all)]
pub async fn connect(url: String) -> eyre::Result<DatabaseConnection> {
    let options = ConnectOptions::new(url)
        .sqlx_logging(true)
        .sqlx_logging_level(LevelFilter::Debug)
        .to_owned();
    let database = Database::connect(options)
        .await
        .wrap_err("failed to connect to the database")?;
    info!("database connected");

    Migrator::up(&database, None)
        .await
        .wrap_err("failed to run database migrations")?;
    info!("database schema up-to-date");

    Ok(database)
}
