use eyre::WrapErr;
use std::env::current_dir;
use std::sync::Arc;
use std::{env, net::SocketAddr, path::PathBuf};

pub struct Config {
    /// The address the server should listen on
    pub address: SocketAddr,
    /// The url of the database to connect to
    pub database_url: String,
    /// The path files should be served from
    pub path: PathBuf,
}

/// Parse the configuration from the database
pub fn load() -> eyre::Result<Arc<Config>> {
    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "info");
    }

    let address = env::var("ADDRESS")
        .unwrap_or_else(|_| String::from("0.0.0.0:3000"))
        .parse()
        .wrap_err("invalid address format")?;
    let database_url = env::var("DATABASE_URL").wrap_err("missing DATABASE_URL in environment")?;

    let current_dir = current_dir()?;
    let path = env::var("BASE_PATH")
        .map(PathBuf::from)
        .unwrap_or(current_dir)
        .canonicalize()?;

    let config = Config {
        address,
        database_url,
        path,
    };
    Ok(Arc::new(config))
}
